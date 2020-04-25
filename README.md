# デプロイ

## 前準備

1. config/aws.template.jsonをconfig/aws.jsonにリネームする
2. config/aws.jsonのinstanceIdListに管理するインスタンスのIDを指定する

## ビルド

```shell script
docker run --rm -v ${PWD}:/code -v ${HOME}/.cargo/registry:/root/.cargo/registry -v ${HOME}/.cargo/git:/root/.cargo/git softprops/lambda-rust 
```


