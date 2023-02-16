#!/usr/bin/env bash

# Usage:
# ./replace-string.sh.sh <from> <to>

if [ "$1" == "" ] || [ "$2" == "" ]
then
  echo "error: Not enough arguments."
  echo "usage: ./replace-string.sh <from> <to>"
  exit
fi

find ./ -iname "*.rs" -type f -exec sed -i "s/$1/$2/g" {} \;
