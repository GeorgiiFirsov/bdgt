#!/bin/zsh

FOLDER=$1
ARCH=$2

OUTPUT=$FOLDER/output

pushd $FOLDER

./configure --host $ARCH --prefix $OUTPUT

#
# Build package
#
make
make install

#
# Adjust pkg-config
#
export PKG_CONFIG_PATH="$OUTPUT:$PKG_CONFIG_PATH"

popd