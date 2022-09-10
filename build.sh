#!/bin/sh
unset ANDROID_HOME
export ANDROID_SDK_ROOT=/home/jschiefer/Android/Sdk
export ANDROID_NDK_ROOT=$ANDROID_SDK_ROOT/ndk/25.1.8937393
cargo apk build
cargo apk build --release
cargo apk run

