#!/bin/bash

# USAGE:
# ./update-version.sh <from> <to>

if [ "$1" == "" ] || [ "$2" == "" ]
then
  echo "error: Not enough arguments."
  echo "usage: ./update-version.sh <from> <to>"
  exit
fi

sed -i "s/$1/$2/g" Cargo.toml
