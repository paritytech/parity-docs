# AGENTS.md — parity-docs

This file is loaded automatically by coding agents (Claude Code, Cursor, Codex, and any tool that respects the `AGENTS.md` convention) when they open this repository. Read it before generating or editing any documentation.

## Scope

This repository holds the Parity Developer Docs, which cover the tools, SDKs, libraries, and technologies built and maintained by Parity Technologies. Ecosystem-wide Polkadot documentation (developers.polkadot.com) is maintained separately by the Polkadot Community Foundation in [`Polkadot-Community-Foundation/polkadot-docs`](https://github.com/Polkadot-Community-Foundation/polkadot-docs). Do not add general Polkadot ecosystem content here; link to that site instead.

## Style and authoring rules

Documentation in this repository follows the **Documentation Style Guide**. The agent-loadable rule set lives in a separate repository so it can be shared across projects and updated independently:

- **Canonical source**: [`paritytech/documentation-style-guide/AGENTS.md`](https://github.com/paritytech/documentation-style-guide/blob/main/AGENTS.md)
- **Prose reference**: [`paritytech/documentation-style-guide/style-guide.md`](https://github.com/paritytech/documentation-style-guide/blob/main/style-guide.md)
- **Reviewer checklist**: [`paritytech/documentation-style-guide/checklist.md`](https://github.com/paritytech/documentation-style-guide/blob/main/checklist.md)

**Read the canonical `AGENTS.md` before authoring.** It front-loads the rules that AI-generated documentation most often violates — list punctuation, bold misuse, em dashes, banned phrases, terminology, code-identifier backticks, identifier consistency, image filenames, and the "no redundant subheadings above lead-in lists" rule. The pre-output checklist at the bottom of that file is the last thing to run before returning a draft.

## Linting

Vale rules and the style guide vocab are vendored (committed) from the canonical [`paritytech/documentation-style-guide`](https://github.com/paritytech/documentation-style-guide) repo into `styles/StyleGuide/` and `styles/config/vocabularies/StyleGuide/`, so linting needs no external fetch. Project-specific vocab lives in `styles/config/vocabularies/Polkadot/`. Project-specific rules live in `styles/Polkadot/` and `styles/Parity/`.

```bash
vale .                                                       # lint
python .github/scripts/check_operator_language.py --all      # list operator-language matches, including snippets
./.github/scripts/sync-styleguide-vale.sh                    # refresh vendored rules + vocab from upstream, then commit
```

CI runs Vale against changed markdown files on every pull request (see `.github/workflows/vale.yml`). Vale findings are advisory. The operator-language check in the same workflow is blocking: it fails a pull request that adds a line matching `styles/Parity/OperatorLanguage.yml`. It exists alongside the Vale rule because Vale skips text that Markdown parses as indented code, which includes admonitions, content tabs, and most snippets.

## Project-specific rules

These supplement (and where they conflict, override) the canonical styleguide:

- **Customer override on heading case**: This site uses Chicago title case per the styleguide default.
- **External links open in a new tab**: this is handled by the MkDocs plugin configured for this site. Do not add `{target=\_blank}` manually — the canonical styleguide also drops the manual rule.
- **Snippet includes**: Reusable code lives in `docs/.snippets/code/` and is included via `--8<-- 'code/path/to/file.ext'`. Do not inline code that already exists as a snippet.
- **Variables**: `docs/variables.yml` holds shared version pins and identifiers. Reference via `{{ variable_name }}`.
- **Authors, not operators**: Parity writes software; it does not run, host, or operate networks, endpoints, faucets, directories, or app builds (see `docs/what-parity-builds.md` and the shared statement in `docs/.snippets/text/parity-role.md`). Never write that Parity runs, hosts, operates, deploys, or provides a live service. Name the actual operator, or describe "a network running <software>" instead.
- **Software, not networks**: Document the software (for example, `pallet-revive` or "a chain running `pallet-revive`"), not a specific live network. Tutorials should run against a local node by default. Take RPC URLs, chain IDs, and faucets from the `networks:` block in `docs/variables.yml` (for example, `{{ networks.local.ws_url }}`) instead of inlining them, and link to [developers.polkadot.com](https://developers.polkadot.com/) for network-specific details.

## Precedence

When rules conflict:

1. **This file** (`parity-docs/AGENTS.md`) — project-specific overrides win.
2. **Canonical Documentation Style Guide** — default.
3. **Google developer documentation style guide** — fallback for anything the Documentation Style Guide does not cover.

## How to use this in an agent session

A typical session opens by reading this file, then fetching the canonical styleguide rules (the file at the GitHub URL above, or a local clone if present). Apply the rules during generation, not just at review time. Run the pre-output checklist from the canonical `AGENTS.md` before returning a draft.
