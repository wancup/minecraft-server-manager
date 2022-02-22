# ビルド

## 前準備

1. config/server.template.jsonをconfig/server.jsonにリネーム
2. config/server.jsonのuriにサーバ管理APIのURIを記載
3. config/server.jsonのapiKeyにAPIへアクセスする際のキーを記載

## Windows
VisualStudioランタイムを静的リンクするため以下の環境変数を有効化してビルドする

``` DOS
(set RUSTFLAGS=-C target-feature=+crt-static) && cargo build --release
```
