#!/bin/bash
set -e
export NODE_ENV=production

export PUBLIC_HOST=fujipod.com
export PUBLIC_PORT=443
export HTTPS_ENABLED=1
export SERVICE_WORKER_ENABLED=0

android_web_dir="../android/build_android"
app_output_dir="../android/build_app"
native_build_dir="../android/rust/target/"

if [ -e "$native_build_dir" ]; then
  find "$native_build_dir" -mindepth 1 -maxdepth 1 -exec rm -r {} \;
fi

# get the build folder from docker
rm -r build/* 2>/dev/null || true
docker-compose cp fe:/opt/app/web/build build

# clean
mkdir -p "$android_web_dir"
mkdir -p ignore
mkdir -p "$app_output_dir"
find "$android_web_dir" -mindepth 1 -maxdepth 1 -exec rm -r {} \;
find "$app_output_dir" -mindepth 1 -maxdepth 1 -exec rm -r {} \;

# get the android build folder from docker
docker-compose cp fe:/opt/app/web/build/app.zip ignore/app.zip
unzip ignore/app.zip -d "$android_web_dir" > /dev/null
rm ignore/app.zip
npx cap sync

keystore_file="production.keystore"

echo -n "password for android/$keystore_file:"
read -s password
echo

source scripts/android/android-env.sh

npx cap build \
  --androidreleasetype AAB \
  --keystorepath $keystore_file \
  --keystorepass "$password" \
  --keystorealiaspass "$password" \
  --keystorealias production \
  android

mv ../android/app/build/outputs/bundle/universalRelease/app-universal-release-signed.aab \
  "$app_output_dir"