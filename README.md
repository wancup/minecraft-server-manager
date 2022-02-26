# Minecraft-Server-Manager

## API (AWS Lambda)

### Build (Arm64)

```shell
$ docker image build -t msm-lambda .
$ docker run -v $PWD:/code -v $HOME/.cargo/registry:/root/.cargo/registry -v $HOME/.cargo/git:/root/.cargo/git msm-lambda 
```

### Deploy

1. upload ```target/api/api.zip```
2. Lambdaの環境変数```MSM_EC2_INSTANCE_ID```に管理するEC2のインスタンスIDを設定する
