#!/usr/bin/env python3
"""Generate client-side redirect stubs from redirects.json.

Writes <site>/<old>/index.html meta-refresh stubs for GitHub Pages, which has
no server-side redirects. redirects.json is the single source of truth (see
handover §3.9).

Internal targets are written as RELATIVE URLs (computed from each stub's own
location) so the redirect resolves correctly whether the site is served at the
root (a custom domain) or under a project-page subpath
(paritytech.github.io/parity-docs/).

Absolute targets (http:// or https://) are written unchanged. Use them to send
removed pages to another site, for example content handed off to
https://developers.polkadot.com/.

Never clobbers a real built page.
"""
import argparse, json, os, html, posixpath, re

ABSOLUTE = re.compile(r"^https?://", re.IGNORECASE)


def load(path):
    with open(path) as f:
        return json.load(f)["data"]


def lead_slash(p):
    return p if p.startswith("/") else "/" + p


def rel_target(old_key, new_value):
    """Relative path from the stub at /<old>/ to the target <new>.

    Absolute http(s) URLs are returned unchanged.
    """
    if ABSOLUTE.match(new_value):
        return new_value
    old = lead_slash(old_key).strip("/")
    new = lead_slash(new_value).strip("/")
    rel = posixpath.relpath(new or ".", old or ".")
    if new_value.endswith("/") and not rel.endswith("/"):
        rel += "/"
    return rel


def write_stubs(entries, site_dir):
    written = skipped = 0
    for e in entries:
        old = lead_slash(e["key"]).strip("/")
        raw = rel_target(e["key"], e["value"])
        target = html.escape(raw)
        # Entities are not decoded inside <script>, so give JS a JSON string.
        js_target = json.dumps(raw).replace("</", "<\\/")
        out_dir = os.path.join(site_dir, old)
        index = os.path.join(out_dir, "index.html")
        if os.path.exists(index):        # real page wins; never overwrite
            skipped += 1
            continue
        os.makedirs(out_dir, exist_ok=True)
        with open(index, "w") as f:
            f.write(
                '<!doctype html><html><head><meta charset="utf-8">'
                '<title>Redirecting…</title>'
                f'<meta http-equiv="refresh" content="0; url={target}">'
                f'<link rel="canonical" href="{target}">'
                '<meta name="robots" content="noindex">'
                f'<script>location.replace({js_target})</script></head>'
                f'<body>Redirecting to <a href="{target}">{target}</a></body></html>'
            )
        written += 1
    print(f"stubs: wrote {written}, skipped {skipped} (real pages)")


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--site-dir", default="site")
    ap.add_argument("--redirects", default="redirects.json")
    a = ap.parse_args()
    write_stubs(load(a.redirects), a.site_dir)
