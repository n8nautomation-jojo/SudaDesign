# Android large-document performance: first-pass fixes

This branch adds targeted performance fixes and instrumentation; it does not claim that all large PSD performance problems have been solved.

- Compressed-tile reads now borrow cached `Arc<Tile>` instead of cloning pixel bytes. Non-RGBA8 row conversion uses the already fetched tile, avoiding redundant lookups.
- The Android per-`Surface` decompression cache is capped at 4 MiB rather than 64 MiB. This is an interim policy, **not** an aggregate memory limit.
- Autosave takes cheap `Arc<Document>` snapshots on the UI thread; a dedicated writer serializes checkpoints asynchronously. Each checkpoint uses new document file paths, with the manifest updated only once every document has been saved. The previous complete checkpoint remains recoverable if an in-flight write fails. Process termination may still prevent a new checkpoint from completing.
- Severe thermal conditions no longer purge caches every frame.
- Periodic `logcat` messages report tile hits/misses, decompressed bytes, composite and upload times, frame times, GPU budget and CPU fallback.
- Do **not** disable GPU memory safety budgets.

Before merging, run `cargo fmt --all -- --check`, `cargo test -p photocraft-raster -p photocraft-android`, `cargo clippy -p photocraft-raster -p photocraft-android --all-targets -- -D warnings`, `cargo xtask layers`, `cargo xtask wasm` and Android device benchmarks with large real PSDs. Run a checkpoint restore round-trip and compare PSD fidelity. Full global memory budgeting and GPU/LOD optimization remain separate follow-up work.
