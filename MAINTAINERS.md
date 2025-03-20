# Maintainers

This document describes some instructions for maintainers. Other contributors and users need not be concerned with this material.

### GitHub instructions

When setting up the repository on GitHub, configure the following settings:

- Under `General` → `Pull Requests`, enable `Automatically delete head branches`.
- Under `Secrets and variables`:
  - Under `Actions`, add the following repository secrets with appropriate values:
    - `CRATES_IO_TOKEN`
    - `DOCKER_PASSWORD`
  - Under `Dependabot`, add the `DOCKER_PASSWORD` repository secret from above.
- Under `Branches`, click `Add a branch ruleset` and configure it as follows:
  - For the ruleset name, you can use the name of the branch: `main`.
  - Set `Enforcement status` to `Active`
  - Under `Targets` → `Target branches`, click `Add target` and select `Include default branch` from the dropdown menu.
  - Under `Rules` → `Branch rules`, check `Require status checks to pass` and configure it as follows before clicking the `Create` button:
    - Enable `Require branches to be up to date before merging`
    - Click the `Add checks` button and add the following (you may need to use the search box to find them):
      - `Build for Linux`
      - `Build for Windows`
      - `Build for macOS`
      - `Install on Ubuntu`
      - `Install on macOS`
      - `Publish a release if applicable`

The GitHub workflow will fail initially because the jobs which test the installer script will not find any release to download. You'll need to bootstrap a release by temporarily removing those jobs or changing them to no-ops. Be aware that the `create-release` job is configured to only run on the `main` branch, so you may also need to temporarily change that depending on which branch you're working on.

### Release instructions

Releasing a new version is a four-step process:

1. Run `(cd integration_tests/typescript_web && npm ci && npm run main && open dist/index.html)` to run the browser-based integration tests. This is the only test suite that doesn't run in the GitHub workflow.
2. Bump the version in `[file:Cargo.toml]`, run `cargo build` to update `[file:Cargo.lock]`, and update `[file:CHANGELOG.md]` with information about the new version. Ship those changes as a single commit.
3. Once the GitHub workflow has finished on the `main` branch, update the version in `[file:install.sh]` to point to the new release.
4. Create a pull request in the `Homebrew/homebrew-core` repository on GitHub to bump the version in [this file](https://github.com/Homebrew/homebrew-core/blob/HEAD/Formula/t/typical.rb).
