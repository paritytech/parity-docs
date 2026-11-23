# Parity Developer Documentation

This repository is the monorepo for the Parity developer documentation, which covers the tools, SDKs, libraries, and technologies built and maintained by [Parity Technologies](https://www.parity.io/). It contains both the documentation content and the MkDocs configuration used to build and serve the site.

The documentation source files are written in [Markdown](https://daringfireball.net/projects/markdown) and generally follow the [Documentation Style Guide](https://github.com/paritytech/documentation-style-guide/blob/main/style-guide.md).

## About This Repository

This repository started as Parity's fork of the Polkadot developer documentation and keeps its full Git history. The [Polkadot Developer Docs](https://developers.polkadot.com) are now maintained by the [Polkadot Community Foundation](https://github.com/Polkadot-Community-Foundation/polkadot-docs). For ecosystem-wide Polkadot documentation, refer to that site.

This repository focuses only on Parity's own tools and technologies. Content inherited from the Polkadot docs is being reviewed and refocused accordingly.

## Repository Structure

```
parity-docs/
├── docs/                   # Documentation content (Markdown, images, snippets)
│   ├── get-started/        # What Parity builds, glossary
│   ├── polkadot-sdk/       # Build chains with the Polkadot SDK
│   ├── smart-contracts/    # pallet-revive contract stack
│   ├── apps/               # Product SDK and its reference
│   ├── nodes/              # Run the node binaries
│   ├── tools/              # Client libraries and developer tools
│   ├── images/
│   ├── .snippets/          # Reusable code and text includes
│   └── variables.yml       # Shared versions and network values
├── hooks/                  # MkDocs hooks (Python)
├── layouts/                # Social card layouts
├── material-overrides/     # Theme overrides and custom CSS
├── scripts/                # Utility scripts
├── styles/                 # Vale linting rules
├── mkdocs.yml              # MkDocs configuration
└── requirements.txt        # Python dependencies
```

## Run the Docs Locally

Follow these steps to run the documentation site locally and preview your changes.

### Clone the Repository

```bash
git clone https://github.com/paritytech/parity-docs.git
cd parity-docs
```

> **Contributing?** If you plan to [contribute](CONTRIBUTING.md), fork this repository first and clone your fork instead.

### Set Up the Environment

Install Python dependencies:

```bash
make install
```

On Windows:

```bat
Makefile.bat install
```

### Install Node.js Dependencies

```bash
npm install
```

### Serve the Site

```bash
make serve
```

On Windows:

```bat
Makefile.bat serve
```

After a successful build, the site is available at http://127.0.0.1:8000 with live reload on file changes.

To pass extra flags to `mkdocs serve`, use the `ARGS` variable:

```bash
make serve ARGS="--watch-theme"
```

On Windows, pass them as a second argument:

```bat
Makefile.bat serve "--watch-theme"
```

> **_NOTE:_** To improve build times, you can:
>
> - Disable the git revision plugin: `export ENABLED_GIT_REVISION_DATE=false`
> - Disable social card generation: `export SOCIAL_CARDS=false`

### Build and Validate

To do a full build and check for errors (mirrors CI):

```bash
make build
```

### Working with the Docs Again

To resume work after closing your terminal:

1. Navigate to the repo root.
2. Run `make serve`.

## Production Site

The site is published at [docs.parity.io](https://docs.parity.io/). `.github/workflows/deploy-pages.yml` builds `main` and deploys it to GitHub Pages.

The domain appears in the following files. Keep them in sync if it changes:

- `site_url` in `mkdocs.yml`, which sets canonical URLs, the sitemap, and the `llms.txt` links.
- `docs_base_url` in `llms_config.json`, which sets the URLs in the AI feed.
- The `Sitemap` line in `docs/robots.txt`, and `BASE_URL` in `.github/workflows/pr-link-checker.yml`.

The custom domain is set in the repository's **Settings** > **Pages** > **Custom domain**. Because the site deploys through a GitHub Actions workflow, GitHub ignores any `CNAME` file in the build. The domain's DNS also needs a `CNAME` record pointing `docs` at `paritytech.github.io`.

## Pull Request Previews

Every pull request that changes the docs gets a rendered preview on Cloudflare Pages. A bot comments the link on the pull request, for example `https://pr-12.parity-docs.pages.dev`, and the preview updates on every push.

Previews use two workflows, so that secrets are never exposed to code from a pull request:

- `.github/workflows/docs-preview-build.yml` builds the site on `pull_request`, with no secrets, so it also works for pull requests from forks.
- `.github/workflows/docs-preview-deploy.yml` deploys the built files to Cloudflare Pages and posts the comment. It never checks out pull request code.

One-time setup for maintainers:

1. Create a Cloudflare API token with the **Cloudflare Pages: Edit** permission. Your account ID is shown in the dashboard, under **Workers & Pages** and on the account home page.
2. Add the repository secrets `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID`.

The first preview run creates the Pages project, named `parity-docs`. To use a different name, set the repository variable `CLOUDFLARE_PAGES_PROJECT`.

Until the secrets are set, the deploy workflow skips the deploy with a warning. Production is unaffected: it deploys to GitHub Pages from `main`.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to contribute to this repository.

Every contribution, whether it's fixing a typo, improving documentation, or adding new content, helps developers get the most out of Parity's tools. Thank you for contributing!

## Disclaimer

### Licensing Disclaimer

The developer documentation provided herein has been collaboratively created and curated by [Web 3.0 Technologies Foundation](https://web3.foundation/), [Parity Technologies Ltd.](https://www.parity.io/), and [PaperMoon Dev SL](https://papermoon.io) (hereinafter referred to as the "Organizations") as a joint effort to support the Polkadot ecosystem.

### Documentation License

The content of these documents is [licensed](LICENSE.md) under the Creative Commons Attribution 4.0 International License ([CC BY 4.0](https://creativecommons.org/licenses/by/4.0/)). You are free to share, copy, redistribute, and adapt the material for any purpose, even commercially, provided that appropriate credit is given to the Organizations, a link to the license is provided, and any changes made are indicated.

### Code License

Any code examples or related software provided in these documents are available under the [MIT License](https://opensource.org/license/mit). This permits anyone to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the code, provided that the original copyright notice and this permission notice are included in all copies or substantial portions of the code.

### General Disclaimer

This developer documentation has been created to support and promote the Polkadot ecosystem. The information presented herein is made available solely for general information purposes. The Organizations do not warrant the accuracy, completeness, or usefulness of this information. Any reliance you place on such information is strictly at your own risk. The Organizations disclaim all liability and responsibility arising from any reliance placed on this information by you or by anyone who may be informed of any of its contents. All statements and/or opinions expressed in these materials are solely the responsibility of the person or entity providing those materials and do not necessarily represent the opinion of the Organizations. The information should not be construed as professional or financial advice of any kind. Advice from a suitably qualified professional should always be sought in relation to any particular matter or circumstance. The information herein may link to or integrate with other websites operated or content provided by third parties, and such other websites may link to this website. The Organizations have no control over any such other websites or their content and will have no liability arising out of or related to such websites or their content. The existence of any such link does not constitute an endorsement of such websites, the content of the websites, or the operators of the websites. These links are being provided to you only as a convenience, and you release and hold the Organizations harmless from any and all liability arising from your use of this information or the information provided by any third-party website or service.
