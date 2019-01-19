#!/bin/sh

BASEDIR=$(dirname "$0")

bindgen --rustified-enum ".*" $BASEDIR/sk_includes.h -o $BASEDIR/src/bindings.rs -- -Iskia/include/c  