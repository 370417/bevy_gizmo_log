# Testing cargo doc locally

Needed because the doc_cfg feature for marking feature flags is nightly-only.

```
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features --no-deps --open
```

Not using doc_auto_cfg at the moment because it seems less stable.
