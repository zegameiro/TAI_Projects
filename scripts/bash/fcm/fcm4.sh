#!/bin/bash

EXECUTABLE="../../../target/debug/fcm"

FILE="../../../data/sequences/sequence4.txt"
K=3
ALPHA=0.01

$EXECUTABLE $FILE -k $K -a $ALPHA