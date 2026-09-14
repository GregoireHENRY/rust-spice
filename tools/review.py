"""Print each wrapper's Rust signature next to what CSPICE says its arguments are.

The compiler checks arity and broad shape, but not whether an output pointer is one number or six,
nor what a cell holds. This is the step where that gets read by eye.
"""
import json, os, re, pathlib, sys
CSPICE = pathlib.Path(os.environ.get('CSPICE_DIR', '')) / 'src' / 'cspice'
names = json.load(open(sys.argv[1]))
raw = pathlib.Path('rust-spice/src/core/raw.rs').read_text()
for n in names:
    m = re.search(rf'pub fn {n}(?:<[^(]*>)?\((.*?)\)\s*(->[^{{]*)?\{{', raw, re.S)
    if not m:
        print(f"### {n}: not found"); continue
    sig = ' '.join((m.group(1) + ' ' + (m.group(2) or '')).split())
    print(f"\n{n}\n  rust: ({sig}")
    path = CSPICE / f'{n}_c.c'
    if path.exists():
        block = re.search(r'-Brief_I/O(.*?)-Detailed_Input', path.read_text(errors='replace'), re.S)
        if block:
            for line in block.group(1).splitlines():
                if re.match(r'\s+\w+\s+[IO]\s', line):
                    print("  c   :", ' '.join(line.split()))
