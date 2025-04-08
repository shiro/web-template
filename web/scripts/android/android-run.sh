#!/bin/sh
set -e

native_build_dir="../android/rust/target/"

if [ -e "$native_build_dir" ]; then
  find "$native_build_dir" -mindepth 1 -maxdepth 1 -exec rm -r {} \;
fi

source scripts/dotenv.sh
# source scripts/android/android-env.sh

if [[ "$FORCE_LOCAL_ANDROID" == "1" ]] || [[ "$FORCE_LOCAL_ANDROID" == "true" ]] || [[ "$NODE_ENV" == "production" ]]; then
  echo "run (local mode)"
  # we already built and synced in webpack
  pnpm cap run --no-sync android
else
  echo "run (dev-server mode)"
  # always build and sync to avoid crashing on initial run
  ./scripts/android/android-build.sh
  pnpm cap run android
fi
