#!/usr/bin/sh

# Highly inspired on:
# https://github.com/rjpower4/spice-sys/blob/master/scripts/getspice.sh
# Copyright (c) MIT 2020 Rolfe Power
#
# Modifications:
#   + directory and tar variables
#   + detection download
#   + remove tar

# $OUT_DIR is created randomly during build by cargo
# I would have liked to build it outside $OUT_DIR but couldn't find a way.

DIR="cspice"
TAR="cspice.tar.Z"
URL="http://naif.jpl.nasa.gov/pub/naif/toolkit//C/PC_Linux_GCC_64bit/packages/cspice.tar.Z"

if [ ! -d $OUT_DIR/$DIR ]; then
    wget $URL -P $OUT_DIR
    tar -xf $OUT_DIR/$TAR -C $OUT_DIR

    mv $OUT_DIR/$DIR/lib/cspice.a $OUT_DIR/$DIR/lib/libcspice.a

    rm $OUT_DIR/$TAR
fi
