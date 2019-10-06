#!/bin/bash

set -ex

cargo build
./scripts/osx_vst_bundler.sh complex_clip ./target/debug/libdistortion_vst.dylib

mv ./complex_clip.vst ~/complex_clip.vst
