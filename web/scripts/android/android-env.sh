#!/bin/sh

# reference: https://github.com/briansmith/ring/blob/main/mk/cargo.sh

android_tools=$ANDROID_SDK_ROOT/ndk/25.1.8937393/toolchains/llvm/prebuilt/linux-x86_64/bin

export CC_aarch64_linux_android=$android_tools/aarch64-linux-android21-clang
export AR_aarch64_linux_android=$android_tools/llvm-ar
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=$android_tools/aarch64-linux-android21-clang

export CC_armv7_linux_androideabi=$android_tools/armv7a-linux-androideabi19-clang
export AR_armv7_linux_androideabi=$android_tools/llvm-ar
export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER=$android_tools/armv7a-linux-androideabi19-clang

# ring does not support this arch... check back if really needed
# export CC_i686-linux-android=$android_tools/i686-linux-android21-clang
# export AR_i686-linux-android=$android_tools/llvm-ar
# export CARGO_TARGET_I686_LINUX_ANDROID_LINKER=$android_tools/i686-linux-android21-clang
