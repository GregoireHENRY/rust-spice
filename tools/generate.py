"""Classify the CSPICE signatures we have not wrapped, and emit declarations for the ones whose
shape the procedural macro already handles.

Anything this cannot classify with confidence is listed for hand writing instead: a wrong guess
here is a silent marshalling bug, which is exactly what the test suite exists to catch.
"""
import collections, json, os, pathlib, re, sys

CSPICE = pathlib.Path(os.environ.get('CSPICE_DIR', '')) / 'src' / 'cspice'
def _bindgen():
    """The generated bindings, wherever the current build put them."""
    found = sorted(pathlib.Path('target').glob('*/build/cspice-sys-*/out/bindgen.rs'))
    if not found:
        raise SystemExit('no bindgen.rs found; run `cargo build` first')
    return found[-1]

BINDGEN = _bindgen()

def signatures():
    src = BINDGEN.read_text()
    out = {}
    for m in re.finditer(r'pub fn (\w+_c)\((.*?)\)(\s*->\s*([^;]+))?;', src, re.S):
        args = [a.strip() for a in re.split(r',(?![^\[]*\])', ' '.join(m.group(2).split())) if a.strip()]
        params = []
        for a in args:
            name, _, ty = a.partition(': ')
            params.append((name.strip(), ty.strip()))
        out[m.group(1)[:-2]] = (params, (m.group(4) or '').strip())
    return out

def declared_dims(name):
    """The array dimensions the C source declares, which bindgen decays to bare pointers.

    `SpiceDouble azlsta[6]` reaches us as `*mut SpiceDouble`, indistinguishable from a pointer to
    one number, so the real size has to come from the source.
    """
    path = CSPICE / f'{name}_c.c'
    if not path.exists():
        return None
    text = path.read_text(errors='replace')
    m = re.search(rf'^\s*(?:void|Spice\w+)\s+{name}_c\s*\(', text, re.M)
    if not m:
        return None
    depth, i = 0, m.end() - 1
    while i < len(text):
        if text[i] == '(':
            depth += 1
        elif text[i] == ')':
            depth -= 1
            if depth == 0:
                break
        i += 1
    params = text[m.end():i]
    dims = {}
    for decl in params.split(','):
        d = re.search(r'(\w+)\s*((?:\[\s*\d*\s*\])+)\s*$', decl.strip())
        if d:
            sizes = [int(x) if x else 0 for x in re.findall(r'\[\s*(\d*)\s*\]', d.group(2))]
            dims[d.group(1)] = sizes
    return dims

def abstract(name):
    """The one line description from the routine's own source file."""
    path = CSPICE / f'{name}_c.c'
    if not path.exists():
        return None
    text = path.read_text(errors='replace')
    m = re.search(r'-Abstract\s*\n(.*?)\n\s*\n', text, re.S)
    if not m:
        return None
    line = ' '.join(m.group(1).split())
    return line[0].upper() + line[1:] if line else None

# C type -> (rust input type, rust output type)
IN = {
    'SpiceDouble': 'f64', 'ConstSpiceDouble': 'f64',
    'SpiceInt': 'i32', 'ConstSpiceInt': 'i32',
    'SpiceBoolean': 'bool', 'ConstSpiceBoolean': 'bool',
    'SpiceChar': 'char',
    '*mut ConstSpiceChar': '&str',
    '*mut [ConstSpiceDouble; 3usize]': '[[f64; 3]; 3]',
    '*mut [ConstSpiceDouble; 6usize]': '[[f64; 6]; 6]',
    '*mut ConstSpicePlane': 'PLANE', '*mut ConstSpiceEllipse': 'ELLIPSE',
    '*mut ConstSpiceDLADescr': 'DLADSC', '*mut ConstSpiceDSKDescr': 'DSKDSC',
}
OUT = {
    '*mut SpiceDouble': 'f64', '*mut SpiceInt': 'i32', '*mut SpiceBoolean': 'bool',
    '*mut SpiceChar': 'String',
    '*mut [SpiceDouble; 3usize]': '[[f64; 3]; 3]',
    '*mut [SpiceDouble; 6usize]': '[[f64; 6]; 6]',
    '*mut SpicePlane': 'PLANE', '*mut SpiceEllipse': 'ELLIPSE',
    '*mut SpiceDLADescr': 'DLADSC', '*mut SpiceDSKDescr': 'DSKDSC',
}
RET = {'SpiceDouble': 'f64', 'SpiceInt': 'i32', 'SpiceBoolean': 'bool'}

# Output pointers to a bare scalar type are ambiguous: a `*mut SpiceDouble` may be one number or an
# array whose length another argument gives. Only treat it as a scalar when nothing suggests a size.
SIZE_HINT = re.compile(r'(len|lenout|ndim|room|maxn|nmax|size|npts|n)$')

def directions(name):
    """Which parameters the routine documents as inputs and which as outputs."""
    path = CSPICE / f'{name}_c.c'
    if not path.exists():
        return None
    block = re.search(r'-Brief_I/O(.*?)-Detailed_Input', path.read_text(errors='replace'), re.S)
    if not block:
        return None
    out = {}
    for line in block.group(1).splitlines():
        m = re.match(r'\s+(\w+)\s+(I-O|I|O|P)\s', line)
        if m:
            out[m.group(1)] = m.group(2)
    return out or None

def sized(base, sizes):
    """Turn a scalar Rust type and a list of array dimensions into the Rust array type."""
    for n in reversed(sizes):
        if n == 0:
            return None
        base = f'[{base}; {n}]'
    return base

def classify(name, params, ret):
    """Return a declaration, or a reason it needs hand writing."""
    dims = declared_dims(name)
    if dims is None:
        return None, 'no source file to take the array dimensions from'
    docdir = directions(name)
    if docdir is None:
        return None, 'no documented argument directions'

    ins, outs = [], []
    for pname, ty in params:
        d = docdir.get(pname) or docdir.get(pname.rstrip('_'))
        if d is None:
            return None, f'{pname} is not in the documented argument list'
        if d == 'I-O':
            return None, f'{pname} is both an input and an output'
        (ins if d == 'I' else outs).append((pname, ty))

    if [p for p, _ in params] != [p for p, _ in ins] + [p for p, _ in outs]:
        return None, 'the outputs do not all come last'
    if any(ty not in OUT for _, ty in outs):
        return None, 'an output is not a plain pointer'
    if ret and ret not in RET:
        return None, f'returns {ret}'
    if ret and outs:
        return None, 'returns a value and writes outputs'

    rin = []
    lenout = None
    for pname, ty in ins:
        if ty in ('*mut ConstSpiceDouble', '*mut ConstSpiceInt'):
            base = 'f64' if 'Double' in ty else 'i32'
            rty = sized(base, dims.get(pname, []))
            if rty is None or rty == base:
                return None, f'input {pname} has no fixed size'
        else:
            rty = IN.get(ty)
        if rty is None:
            return None, f'input {pname}: {ty}'
        if rty == '&str' and SIZE_HINT.search(pname):
            return None, f'ambiguous size argument {pname}'
        rin.append((pname, rty))
    for pname, ty in ins:
        if ty in ('SpiceInt',) and pname in ('lenout', 'outlen', 'srflen', 'sclklen', 'namelen'):
            lenout = pname

    # An output declared as a bare pointer next to an integer output that counts things is an
    # array, not one number: `bodvar_c(body, item, dim, values)` writes `dim` doubles through a
    # `SpiceDouble *`, which is indistinguishable from a scalar in the signature alone.
    counter = {'dim', 'n', 'nvals', 'npts', 'count', 'number', 'nelt', 'nfetch'}
    sizer = {'begin', 'end', 'first', 'last', 'room', 'maxn', 'nmax', 'size', 'ndim', 'lenvals'}
    unsized = [pn for pn, ty in outs
               if ty in ('*mut SpiceDouble', '*mut SpiceInt') and not dims.get(pn)]
    if unsized and any(pn in counter and ty == '*mut SpiceInt' for pn, ty in outs):
        return None, 'array output sized by a companion count'
    if unsized and any(pn in sizer for pn, _ in ins):
        return None, 'array output sized by an input argument'

    rout = []
    for pname, ty in outs:
        if ty in ('*mut SpiceDouble', '*mut SpiceInt'):
            base = 'f64' if 'Double' in ty else 'i32'
            sizes = dims.get(pname, [])
            rty = base if not sizes else sized(base, sizes)
            if rty is None:
                return None, f'output {pname} has no fixed size'
        elif ty == '*mut SpiceChar':
            if dims.get(pname):
                return None, f'output {pname} is an array of strings'
            rty = 'String'
        else:
            rty = OUT[ty]
        rout.append(rty)
    if 'String' in rout and lenout is None:
        return None, 'string output with no length argument'

    return (rin, rout, ret, lenout), None

def rust_name(p):
    return {'in_': 'input', 'ref_': 'frame', 'type_': 'kind', 'match_': 'pattern',
            'strCase': 'strcase', 'return_': 'returned', 'str_': 'string',
            'out': 'output', 'box_': 'boxed'}.get(p, p)

def declaration(name, rin, rout, ret, lenout, doc):
    args = ', '.join(
        f'#[lenout] {rust_name(p)}: {t}' if p == lenout else f'{rust_name(p)}: {t}'
        for p, t in rin)
    if ret:
        out = f' -> {RET[ret]}'
        attrs = '    #[return_output]\n'
    elif len(rout) == 1:
        out, attrs = f' -> {rout[0]}', ''
    elif rout:
        out, attrs = f' -> ({", ".join(rout)})', ''
    else:
        out, attrs = '', ''
    if len(rin) >= 7:
        attrs = '    #[allow(clippy::too_many_arguments)]\n' + attrs
    if len(rout) >= 5:
        attrs = '    #[allow(clippy::type_complexity)]\n' + attrs
    lock = '' if lenout else '    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]\n'
    return (f'cspice_proc! {{\n    /**\n    {doc}\n    */\n{attrs}{lock}'
            f'    pub fn {name}({args}){out} {{}}\n}}\n')

def main():
    sigs = signatures()
    doc = pathlib.Path('rust-spice/src/core/mod.rs').read_text()
    table = doc[doc.index('## Bindings'):doc.index('\n\n[')]
    ours = {m.group(1) for m in (re.match(r'\[(\w+)_c\]\[', l) for l in table.splitlines()) if m}
    want = set(json.load(open(sys.argv[1]))) if len(sys.argv) > 1 else set(sigs)

    ok, skip = {}, collections.Counter()
    for name in sorted(want - ours):
        if name not in sigs:
            continue
        params, ret = sigs[name]
        decl, why = classify(name, params, ret)
        if decl is None:
            skip[why] += 1
            continue
        a = abstract(name) or f'Wrapper for `{name}_c`.'
        ok[name] = (declaration(name, *decl, a), a)
    print(f"// macro-able: {len(ok)} of {len(want - ours)}", file=sys.stderr)
    for why, n in skip.most_common(12):
        print(f"//   needs hand writing ({n}): {why}", file=sys.stderr)
    print('\n'.join(d for d, _ in ok.values()))

main()
