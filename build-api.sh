cd /code || return

mkdir -p target/api
export CARGO_TARGET_DIR=$PWD/target/api

cargo build --release -p api
cd target/api || return
rm -f api.zip
zip -j api.zip release/bootstrap
cd ../../
