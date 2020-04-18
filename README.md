# ビルド

## Windows
VisualStudioランタイムを静的リンクするため以下の環境変数を有効化してビルドする

``` DOS
(set RUSTFLAGS=-C target-feature=+crt-static) && cargo build --release
```
