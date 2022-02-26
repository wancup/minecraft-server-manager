

## API (AWS Lambda)

### Preparation

1. api/config/aws.template.jsonをapi/config/aws.jsonにコピー
2. api/config/aws.jsonのinstanceIdに管理するインスタンスのIDを指定する

### Build (Arm64)

```shell
$ docker image build -t msm-lambda .
$ docker run -v $PWD:/code -v $HOME/.cargo/registry:/root/.cargo/registry -v $HOME/.cargo/git:/root/.cargo/git msm-lambda 
```

### Deploy

upload ```target/api/api.zip```
