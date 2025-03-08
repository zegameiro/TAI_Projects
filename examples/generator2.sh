#!/bin/bash

EXECUTABLE="target/debug/generator"

FILE="sequences/sequence2.txt"
K=5
ALPHA=0.1
PRIOR="armas"
SEQUENCE_LENGTH=100

$EXECUTABLE $FILE -k $K -a $ALPHA -p $PRIOR -s $SEQUENCE_LENGTH
