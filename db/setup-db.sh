#!/bin/sh

# diesel migration generate

basedir=$(dirname "$0")

(
    cd $basedir
    diesel setup
    diesel migration run
)
