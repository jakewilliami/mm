#!/usr/bin/env sh

set -xe
trap 'exit 1' INT

set -xe

gcc -o main main.m -framework Cocoa
