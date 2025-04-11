#!/bin/bash

EXECUTABLE="../../../target/debug/generator"

FILE="../../../data/sequences/sequence5.txt"
K=300
ALPHA=0.2
PRIOR="for"
SEQUENCE_LENGTH=500

$EXECUTABLE $FILE -k $K -a $ALPHA -p $PRIOR -s $SEQUENCE_LENGTH
