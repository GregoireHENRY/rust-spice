"""Flag any wrapper whose Rust outputs do not match what the routine documents.

`#[return_output]` wrappers have no documented `O` parameters, because the C routine returns the
value instead of writing it, so they are counted separately.
"""
import json, os, re, pathlib, sys
CSPICE = pathlib.Path(os.environ.get('CSPICE_DIR', '')) / 'src' / 'cspice'
raw = pathlib.Path('rust-spice/src/core/raw.rs').read_text()
bad = 0
for n in json.load(open(sys.argv[1])):
    block = next((b for b in raw.split('cspice_proc! {') if re.search(rf'\n    pub fn {n}\(', b)), None)
    if block is None:
        print(f"{n}: not generated"); continue
    block = block[:block.index(f'pub fn {n}(') + 400]
    returns_value = '#[return_output]' in block
    m = re.search(rf'pub fn {n}\((.*?)\)\s*(->[^{{]*)?\{{\}}', block, re.S)
    ret = (m.group(2) or '').lstrip('-> ').strip()
    n_rust = 0 if not ret else (ret.count(',') + 1 if ret.startswith('(') else 1)
    path = CSPICE / f'{n}_c.c'
    block = re.search(r'-Brief_I/O(.*?)-Detailed_Input', path.read_text(errors='replace'), re.S)
    docs = [l for l in block.group(1).splitlines() if re.match(r'\s+\w+\s+O\s', l)]
    expected = 1 if returns_value else len(docs)
    if n_rust != expected:
        bad += 1
        print(f"MISMATCH {n}: rust {n_rust}, documented {expected}")
print(f"checked {len(json.load(open(sys.argv[1])))}, mismatches {bad}")
