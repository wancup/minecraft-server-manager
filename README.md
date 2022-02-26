# Minecraft-Server-Manager

マイクラを遊ぶときだけAmazon EC2のサーバを起動するためのAWS Lambda関数とGUIクライアント

## Prerequisites

* Rust (>= 1.56.0)
* Docker

## EC2

1. マイクラサーバーのjarファイルをEC2に配置
2. サーバーの初回起動とEULAの設定を行う
3. ```start_mc_server.sh```をEC2に置き、インスタンス起動時に実行するよう設定

## API (AWS Lambda)

### Build (Arm64)

```shell
$ docker image build -t msm-lambda .
$ docker run -v $PWD:/code -v $HOME/.cargo/registry:/root/.cargo/registry -v $HOME/.cargo/git:/root/.cargo/git msm-lambda 
```

### Deploy

1. upload ```target/api/api.zip```
2. Lambdaの環境変数```MSM_EC2_INSTANCE_ID```に管理するEC2のインスタンスIDを設定
3. API Gatewayを設定

## Client

### Preparation

1. client/config/server.template.jsonをconfig/server.jsonにリネーム
2. client/config/server.jsonのuriにサーバ管理APIのURIを記載
3. client/config/server.jsonのapiKeyにAPIへアクセスする際のキーを記載

### Build

```shell
cargo build -p client --release
```

#### Windows

VisualStudioランタイムを静的リンクするため以下の環境変数を有効化してビルドする

```DOS
(set RUSTFLAGS=-C target-feature=+crt-static) && cargo build -p client --release
```

