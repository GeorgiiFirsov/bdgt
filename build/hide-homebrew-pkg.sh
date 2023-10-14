#!/bin/zsh

# !!! WARNING !!!
# This script is destructive and targets only temporary CI builders!
# Do not use it on real machine!

PACKAGE=$1
CFG_ROOT=/opt/homebrew/lib/pkgconfig
EXTENSION=.pc
PREFIX=bkp-

ORIGINAL=$CFG_ROOT/$PACKAGE$EXTENSION
MODIFIED=$PREFIX$ORIGINAL

if [ -f $ORIGINAL ]; then
    echo Renaming "$ORIGINAL" to "$MODIFIED"
    mv $ORIGINAL $MODIFIED
fi
