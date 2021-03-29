#!/usr/bin/sh

# Highly inspired on:
# https://github.com/rjpower4/spice-sys/blob/master/scripts/getspice.sh
# Copyright (c) MIT 2020 Rolfe Power
#
# Modifications:
#   + directory and tar variables
#   + detection download
#   + remove tar

DIR="."
TAR="cspice.tar.Z"
URL="http://naif.jpl.nasa.gov/pub/naif/toolkit//C/PC_Linux_GCC_64bit/packages/cspice.tar.Z"

if [ ! -d "$DIR" ]; then
    mkdir $DIR

    wget $URL -P $DIR
    tar -xf $DIR/$TAR -C $DIR

    mv $DIR/cspice/lib/cspice.a $DIR/cspice/lib/libcspice.a

    rm $DIR/$TAR
fi
