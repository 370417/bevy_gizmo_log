# Release process

- [ ] Update version number in Cargo.toml.
- [ ] Update compatible versions table in lib.rs doc comment.
- [ ] Update compatible versions table in README.md.
- [ ] Update Changelog.md.
- [ ] Make sure Github actions build is passing.
- [ ] Make sure the basic example looks good.
      It should look like a hexagonal color wheel inside a circle.
      `cargo run --example basic`
- [ ] Publish to crates.io:
      `cargo publish --dry-run`
      `cargo publish`
- [ ] Tag the release commit.
      `git tag -a v0.0.0 -m "Release v0.0.0"`
      `git push --tags`
- [ ] Create a release in Github.

# Testing cargo doc locally

Needed because the doc_cfg feature for marking feature flags is nightly-only.

```
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features --no-deps --open
```

Not using doc_auto_cfg at the moment because it seems less stable.
