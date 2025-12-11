#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -x

git add .
git commit -m .

cd example

cargo fmt

cargo +nightly clippy --tests --all-targets --all-features --fix -Z unstable-options -- -A clippy::uninit_assumed_init

cargo run

cargo run --example err
