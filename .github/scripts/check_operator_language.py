#!/usr/bin/env python3
"""Flag wording that presents Parity as an operator of live services.

Parity writes software; it does not run, host or operate networks, endpoints,
faucets, directories or app builds (see "Authors, not operators" in AGENTS.md).

The patterns live in styles/Parity/OperatorLanguage.yml so Vale (editor
feedback) and this script (CI enforcement) share one source. This script exists
because Vale skips text that Markdown parses as indented code, which includes
MkDocs admonitions, content tabs and most snippets.

Usage:
  python .github/scripts/check_operator_language.py --all
      List every match under docs/ (including docs/.snippets/).
  python .github/scripts/check_operator_language.py --diff BASE HEAD
      Check only lines added between two commits; exit 1 on any match.
"""

import argparse
import re
import subprocess
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]
RULE = ROOT / "styles" / "Parity" / "OperatorLanguage.yml"
DOCS = ROOT / "docs"
MARKUP = re.compile(r"\*\*|__|`")
LINK = re.compile(r"\[([^\]]*)\]\([^)]*\)")


def load_patterns():
    rule = yaml.safe_load(RULE.read_text(encoding="utf-8"))
    flags = re.IGNORECASE if rule.get("ignorecase") else 0
    return [re.compile(t, flags) for t in rule["tokens"]]


def matches(line, patterns):
    text = MARKUP.sub("", LINK.sub(r"\1", line))
    return [m.group(0) for p in patterns for m in p.finditer(text)]


def scan_all(patterns):
    hits = []
    for path in sorted(DOCS.rglob("*.md")):
        for n, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
            for m in matches(line, patterns):
                hits.append((path.relative_to(ROOT), n, m))
    return hits


def scan_diff(patterns, base, head):
    diff = subprocess.run(
        ["git", "diff", "--unified=0", "--no-color", f"{base}...{head}", "--", "docs/*.md", "docs/**/*.md"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    ).stdout
    hits, path, n = [], None, 0
    for line in diff.splitlines():
        if line.startswith("+++ "):
            path = line[6:] if line.startswith("+++ b/") else None
        elif line.startswith("@@"):
            n = int(re.search(r"\+(\d+)", line).group(1)) - 1
        elif line.startswith("+") and path:
            n += 1
            for m in matches(line[1:], patterns):
                hits.append((path, n, m))
    return hits


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--all", action="store_true", help="scan every Markdown file under docs/")
    g.add_argument("--diff", nargs=2, metavar=("BASE", "HEAD"), help="scan lines added between two commits")
    args = ap.parse_args()

    patterns = load_patterns()
    hits = scan_all(patterns) if args.all else scan_diff(patterns, *args.diff)
    for path, n, m in hits:
        print(f"{path}:{n}: '{m}'")
    if hits:
        print(f"\n{len(hits)} match(es). Parity is an author, not an operator: name the operator, "
              "or describe the software instead.", file=sys.stderr)
    return 1 if hits and args.diff else 0


if __name__ == "__main__":
    sys.exit(main())
