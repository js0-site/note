#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -x

if [ -d "$1" ]; then
  echo "$1 EXIST"
  exit 1
fi

cp -R _tmpl $1
