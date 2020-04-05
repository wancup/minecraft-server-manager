# デプロイ

## ビルド

```shell script
docker run --rm -v ${PWD}:/code -v ${HOME}/.cargo/registry:/root/.cargo/registry -v ${HOME}/.cargo/git:/root/.cargo/git softprops/lambda-rust 
```


