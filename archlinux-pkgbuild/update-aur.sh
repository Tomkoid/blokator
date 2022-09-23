#!/bin/bash

# USAGE:
# ./update-aur.sh <version>

CD_DIR="$PWD"

REPO_URL="ssh://aur@aur.archlinux.org/blokator.git"

if [ ! -d "blokator-aur" ]
then
  git clone $REPO_URL blokator-aur
fi

# Copy the package build file
cp PKGBUILD blokator-aur/

cd blokator-aur
makepkg --printsrcinfo > .SRCINFO

git add .SRCINFO PKGBUILD
git commit -S -am "Update to $1"
git push $REPO_URL master

cd $CD_DIR
