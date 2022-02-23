

## API (AWS Lambda)

### Build (Arm64)

```shell
$ docker image build -t msm-lambda .
$ docker run -v $PWD:/code -v $HOME/.cargo/registry:/root/.cargo/registry -v $HOME/.cargo/git:/root/.cargo/git msm-lambda 
```

### Deploy

upload ```target/api/api.zip```
