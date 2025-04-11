#!/bin/bash

EXECUTABLE="../../../target/debug/generator"

FILE="../../../data/sequences/sequence3.txt"
K=3
ALPHA=0.1
PRIOR="TAI"
SEQUENCE_LENGTH=100

$EXECUTABLE $FILE -k $K -a $ALPHA -p $PRIOR -s $SEQUENCE_LENGTH
