#!/bin/bash
VERSION=$@
echo "Version: $VERSION"
echo $VERSION > dist/VERSION
sed -i "s/window\.__CELER_VERSION='.*'/window\.__CELER_VERSION='$VERSION'/" dist/app/view.html
sed -i "s/window\.__CELER_VERSION='.*'/window\.__CELER_VERSION='$VERSION'/" dist/app/edit.html
