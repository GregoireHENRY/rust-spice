#!/usr/bin/sh

# Highly inspired on:
# https://github.com/rjpower4/spice-sys/blob/master/scripts/getspice.sh
# Copyright (c) MIT 2020 Rolfe Power
#
# Modifications:
#   + directory and tar variables
#   + detection download
#   + remove tar

DIR="cspice"
TAR="cspice.tar.Z"
URL="http://naif.jpl.nasa.gov/pub/naif/toolkit//C/PC_Linux_GCC_64bit/packages/cspice.tar.Z"

if [ ! -d $DIR ]; then
    wget $URL
    tar -xf $TAR

    mv $DIR/lib/cspice.a $DIR/lib/libcspice.a

    rm $TAR
fi
