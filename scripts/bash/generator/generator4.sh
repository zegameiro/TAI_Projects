#!/bin/bash

EXECUTABLE="../../../target/debug/generator"

FILE="../../../data/sequences/sequence4.txt"
K=3
ALPHA=0.1
PRIOR="AT"
SEQUENCE_LENGTH=20

$EXECUTABLE $FILE -k $K -a $ALPHA -p $PRIOR -s $SEQUENCE_LENGTH
