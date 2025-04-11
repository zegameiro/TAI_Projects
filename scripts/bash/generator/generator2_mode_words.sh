#!/bin/bash

EXECUTABLE="../../../target/debug/generator"

FILE="../../../data/sequences/sequence2.txt"
K=3
ALPHA=0.1
PRIOR="armas"
SEQUENCE_LENGTH=200

$EXECUTABLE $FILE -k $K -a $ALPHA -p $PRIOR -s $SEQUENCE_LENGTH -m words
