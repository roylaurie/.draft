#!/bin/bash
set -o errexit -o pipefail -o privileged -o nounset 
cd "$(npm prefix)"

FROG_VERSION="$(cat etc/frog-version.txt)"
FROG_ARCH="amd64"

echo -e "building bullfrog ...\n"

rm -rf ./dist
mkdir -p ./dist/bullfrog

_rsyncExcludes="--exclude "*.swp" --exclude="*.swo""
rsync -a ./bash ./dist/bullfrog $_rsyncExcludes
rsync -a ./etc ./dist/bullfrog $_rsyncExcludes
rsync -a ./js ./dist/bullfrog $_rsyncExcludes
rsync -a ./json ./dist/bullfrog $_rsyncExcludes
rsync -a ./file ./dist/bullfrog $_rsyncExcludes

# replace source json where minified
mv ./dist/bullfrog/json/data/package/modules.min.json ./dist/bullfrog/json/data/package/modules.json
mv ./dist/bullfrog/json/schema/package/modules.schema.min.json ./dist/bullfrog/json/schema/package/modules.schema.json

# build the debian package
mkdir -p ./dist/debian/bullfrog/usr/local/lib ./dist/debian/bullfrog/usr/local/bin
rsync -a ./tool/file/packaging/debian/DEBIAN ./dist/debian/bullfrog $_rsyncExcludes
rsync -a ./dist/bullfrog ./dist/debian/bullfrog/usr/local/lib $_rsyncExcludes
ln -s ../lib/bullfrog/bash/bin/bullfrog.bash ./dist/debian/bullfrog/usr/local/bin/bullfrog

dpkgFilename="bullfrog-${FROG_VERSION}-${FROG_ARCH}.deb"

dpkg-deb --build ./dist/debian/bullfrog
mv ./dist/debian/bullfrog.deb ./dist/$dpkgFilename
cp -f ./dist/$dpkgFilename ./file/debian-package
cp -f ./dist/$dpkgFilename ./dist/bullfrog/file/debian-package
cp -f ./dist/$dpkgFilename ./dist/debian/bullfrog/usr/local/lib/bullfrog/file/debian-package

# run a second time to include a copy of the deb file
dpkg-deb --build ./dist/debian/bullfrog
mv ./dist/debian/bullfrog.deb ./dist/$dpkgFilename
cp -f ./dist/$dpkgFilename ./file/debian-package
cp -f ./dist/$dpkgFilename ./dist/bullfrog/file/debian-package
cp -f ./dist/$dpkgFilename ./dist/debian/bullfrog/usr/local/lib/bullfrog/file/debian-package

rm -rf ./dist/debian

echo
echo "build successful"
