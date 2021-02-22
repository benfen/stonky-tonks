#!/bin/sh

basedir=$(dirname "$0")

(
    cd $basedir
    diesel setup
    diesel migration run
)
