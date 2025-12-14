#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -x

git_commit() {
  git add . && git commit -m . || true
}
cd ..
git_commit

run() {
  cd $DIR/$1
  cargo fmt
  git_commit
  cargo +nightly clippy --tests --all-targets --all-features --fix -Z unstable-options -- -A clippy::uninit_assumed_init
  cargo run
}

run main
