#!/usr/bin/env sh

set -xe
trap 'exit 1' INT

gcc -o mm main.m -framework Cocoa
