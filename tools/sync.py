"""Regenerate the re-export lists, the bindings table and the link definitions.

Reads `{"name": "description", ...}` for the new rows on stdin; everything else is derived from
the source, so the table cannot drift from what the crate actually exports.
"""
import json, re, pathlib, sys, textwrap

ROOT = pathlib.Path('rust-spice/src')
new_rows = json.load(sys.stdin) if not sys.stdin.isatty() else {}

# Wrappers written inside a macro are not literal `pub fn`s, so each generating macro is listed
# with the pattern that picks the names out of its invocation. `None` marks a macro whose expansion
# does carry a `pub fn`, and an unlisted one stops the script rather than quietly dropping its
# functions from the exports.
BRACED = {
    'ck_record': r'^\s*(\w+) =>',
    'ck_record_count': r'^\s*(\w+) =>',
    'constants': r'^\s*(\w+) =>',
    'cspice_proc': None,
    'daf_read': r'^\s*(\w+) =>',
    'das_add': r'^\s*(\w+)\(',
    'das_read': r'^\s*(\w+)\(',
    'das_update': r'^\s*(\w+)\(',
    'clear_array': r'^\s*(\w+)\(',
    'dsk_fetch': r'^\s*(\w+)\(',
    'extremum': r'^\s*(\w+)\(',
    'fill_array': r'^\s*(\w+)\(',
    'gf_search': r'^\s*(\w+)\(',
    'inout_routine': r'^\s*(\w+) =>',
    'linear_search': r'^\s*(\w+)\(',
    'matrix_g': r'^\s*(\w+) =>',
    'order_of': r'^\s*(\w+)\(',
    'ordinal': r'^\s*(\w+)\(',
    'reorder': r'^\s*(\w+)\(',
    'scalar_g': r'^\s*(\w+)\(',
    'search': r'^\s*(\w+)\(',
    'search_strings': r'^\s*(\w+) =>',
    'sort_in_place': r'^\s*(\w+)\(',
    'spk_chebyshev': r'^\s*(\w+) =>',
    'spk_states_at_epochs': r'^\s*(\w+)\(',
    'spk_states_evenly_spaced': r'^\s*(\w+) =>',
    'vector_g': r'^\s*(\w+)\(',
}

# Macros called once per wrapper, with the name of the wrapper as their first argument.
CALLED = ('ek_add_column', 'ek_record_read', 'ek_record_write', 'ek_record_write_c')


def public(mod):
    text = (ROOT / 'core' / f'{mod}.rs').read_text()
    fns = set(re.findall(r'^pub fn (\w+)', text, re.M)) | set(re.findall(r'^    pub fn (\w+)', text, re.M))
    for match in re.finditer(r'^(\w+)! \{$', text, re.M):
        macro = match.group(1)
        if macro not in BRACED:
            raise SystemExit(f'{mod}.rs: {macro}! generates wrappers this script does not know about')
        if BRACED[macro] is None:
            continue
        block = text[match.end():]
        fns |= set(re.findall(BRACED[macro], block[:block.index('\n}')], re.M))
    for macro in CALLED:
        fns |= set(re.findall(rf'^{macro}!\(\s*\n\s*(\w+),', text, re.M))
    return (fns,
            set(re.findall(r'^pub const (\w+)', text, re.M)),
            set(re.findall(r'^pub type (\w+)', text, re.M)))


neat_fns, _, _ = public('neat')
raw_fns, raw_consts, raw_types = public('raw')
err_fns, _, _ = public('errors')
raw_fns -= neat_fns
raw_consts.discard('CELL_MAXID')

def use(header, names):
    body = ', '.join(sorted(names, key=lambda n: (n.isupper(), n)))
    return header + '\n' + textwrap.fill(body + ',', width=96,
                                         initial_indent='    ', subsequent_indent='    ') + '\n};'

mod_path = ROOT / 'core' / 'mod.rs'
s = mod_path.read_text()
s = re.sub(r'pub use self::neat::\{.*?\n\};', use('pub use self::neat::{', neat_fns), s, flags=re.S)
s = re.sub(r'pub use self::raw::\{.*?\n\};',
           use('pub use self::raw::{', raw_fns | raw_consts | raw_types), s, flags=re.S)

# --- table -------------------------------------------------------------------------------------
start = s.index('CSPICE | **rust-spice** | Description')
end = s.index('\n\n[', start)
lines = s[start:end].splitlines()
owner = {n: 'neat' for n in neat_fns} | {n: 'raw' for n in raw_fns} | {n: 'errors' for n in err_fns}
for name, desc in new_rows.items():
    mod = owner[name]
    lines.append(f'[{name}_c][{name}_c link] | [`{mod}::{name}`] | {desc}')
s = s[:start] + '\n'.join(lines[:2] + sorted(lines[2:])) + s[end:]

listed = {m.group(1) for m in (re.match(r'\[(\w+)_c\]\[', l) for l in lines[2:]) if m} | set(new_rows)
link_block = '\n'.join(
    f'[{n}_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/{n}_c.html'
    for n in sorted(listed))
s = re.sub(r'\[\w+_c link\]: https://naif[^\n]*\n(\[\w+_c link\]: https://naif[^\n]*\n?)*',
           link_block + '\n', s, count=1)
mod_path.write_text(s)

# --- constants also reachable when `lock` hides `raw` --------------------------------------------
lib_path = ROOT / 'lib.rs'
s = lib_path.read_text()
marker = '// GENERATED: the constants of `raw`, which `lock` would otherwise put out of reach.\n'
s = re.sub(re.escape(marker) + r'pub use crate::core::raw::\{.*?\n\};',
           marker + use('pub use crate::core::raw::{',
                        sorted(re.findall(r'^pub const (\w+)', (ROOT/'core'/'raw.rs').read_text(), re.M))),
           s, flags=re.S)
lib_path.write_text(s)
print(f"neat {len(neat_fns)}, raw {len(raw_fns)} fn, table {len(listed)} rows")
