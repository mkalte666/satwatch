#!/bin/bash

DL_DIR=$(readlink -f "target/cspice/download")
INSTALL_DIR=$(readlink -f "target/cspice/install")

CSPICE_URL="https://naif.jpl.nasa.gov/pub/naif/toolkit//C/PC_Linux_GCC_64bit/packages/cspice.tar.Z"
ZIP_NAME="$DL_DIR/cpsice.tar.Z"

#rm -rf target/cspice

#mkdir -p "$DL_DIR"
#mkdir -p "$INSTALL_DIR"

#curl "$CSPICE_URL" -o "$ZIP_NAME"

(
  cd "$INSTALL_DIR" || exit 1
  zcat $ZIP_NAME | tar -xvf -
  cd "$INSTALL_DIR/cspice" || exit 1
  cp lib/cspice.a lib/libcspice.a
  cp lib/csupport.a lib/libcsupport.a
)

