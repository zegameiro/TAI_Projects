#!/bin/bash

EXECUTABLE="../../../target/debug/generator"

FILE="../../../data/sequences/sequence1.txt"
K=3
ALPHA=0.01
PRIOR="ACTG"
SEQUENCE_LENGTH=200

$EXECUTABLE $FILE -k $K -a $ALPHA -p $PRIOR -s $SEQUENCE_LENGTH
