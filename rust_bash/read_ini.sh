#!/bin/bash
INI_FILE=$1
SECTION=$2
KEY=$3

grep -A 1 "^\[$SECTION\]" $INI_FILE | grep "^$KEY=" | cut -d '=' -f2