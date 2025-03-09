#!/bin/bash

EXECUTABLE="target/debug/generator"

FILE="sequences/sequence1.txt"
K=10
ALPHA=0.1
PRIOR="G"
SEQUENCE_LENGTH=10

$EXECUTABLE $FILE -k $K -a $ALPHA -p $PRIOR -s $SEQUENCE_LENGTH
