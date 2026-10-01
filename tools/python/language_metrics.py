import os
import sys

EXTENSIONS = {
    '.c': 'C',
    '.h': 'C',
    '.rs': 'Rust',
    '.S': 'Assembly',
    '.s': 'Assembly',
    '.asm': 'Assembly',
    '.sh': 'Shell',
    '.py': 'Python',
    'Makefile': 'Make',
}

EXCLUDE_DIRS = {
    '.git',
    'build',
    'bin',
    'target',
    'node_modules',
    '.gemini',
    'scratch',
    '.cargo'
}

def analyze_repository(root_dir):
    stats = {
        'C': {'files': 0, 'loc': 0},
        'Rust': {'files': 0, 'loc': 0},
        'Assembly': {'files': 0, 'loc': 0},
        'Shell': {'files': 0, 'loc': 0},
        'Python': {'files': 0, 'loc': 0},
        'Make': {'files': 0, 'loc': 0},
    }

    for dirpath, dirnames, filenames in os.walk(root_dir):
        dirnames[:] = [d for d in dirnames if d not in EXCLUDE_DIRS]
        for f in filenames:
            ext = os.path.splitext(f)[1]
            lang = None
            if f == 'Makefile':
                lang = 'Make'
            elif ext in EXTENSIONS:
                lang = EXTENSIONS[ext]

            if lang:
                file_path = os.path.join(dirpath, f)
                try:
                    with open(file_path, 'r', encoding='utf-8', errors='ignore') as fp:
                        lines = len(fp.readlines())
                    stats[lang]['files'] += 1
                    stats[lang]['loc'] += lines
                except Exception:
                    pass

    total_loc = sum(s['loc'] for s in stats.values())
    return stats, total_loc

def main():
    root = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
    stats, total_loc = analyze_repository(root)

    print("=============================================================")
    print("                PYXISOS LANGUAGE COMPOSITION                 ")
    print("=============================================================")
    targets = {
        'C': 38.0,
        'Rust': 40.0,
        'Assembly': 18.0,
        'Shell': 10.0,
        'Python': 3.0,
        'Make': 2.0,
    }

    for lang in ['C', 'Rust', 'Assembly', 'Shell', 'Python', 'Make']:
        loc = stats[lang]['loc']
        pct = (loc / total_loc * 100.0) if total_loc > 0 else 0.0
        target = targets.get(lang, 0.0)
        diff = pct - target
        print(f"\n{lang}:")
        print(f"  Files    : {stats[lang]['files']}")
        print(f"  LOC      : {loc}")
        print(f"  Actual % : {pct:.1f}%")
        print(f"  Target % : {target:.1f}%")
        print(f"  Variance : {diff:+.1f}%")

    print("\n-------------------------------------------------------------")
    print(f"Total Systems Code LOC: {total_loc}")
    print("=============================================================")

if __name__ == "__main__":
    main()
