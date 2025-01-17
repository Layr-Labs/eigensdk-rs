# Release Process

> [!WARNING]
> 🏗️ This document is a Work-in-Progress 🚧

- [ ] Bump the version in the workspace's `Cargo.toml`, careful of following [Semantic Versioning](https://semver.org/spec/v2.0.0.html), and create a new PR with the change.

- [ ] Once the PR's merged, go to the "Draft a new release" page ([link](https://github.com/Layr-Labs/eigensdk-rs/releases/new)).

- [ ] Generate a new tag for the release by pressing on "Choose a tag", writing the version number `vX.Y.Z`, and pressing on "Create new tag: `vX.Y.Z`".

- [ ] Adjust the release target or previous tag if needed.

- [ ] Press on "Generate release notes" and copy the generated changelog (from the "Added" section to before "New Contributors"), and paste it in `CHANGELOG.md`.

- [ ] Add descriptions for the items in each of the sections as needed. It's recommended that every item that's not in "Documentation" or "Other Changes" has a description of what changed and instructions on how to migrate. You can encounter examples in previous changelog entries.

- [ ] Commit the changelog entry and submit a PR.

- [ ] Once it's merged, update the release draft with the descriptions and save the draft.

- [ ] Once the draft is reviewed, publish the release.
