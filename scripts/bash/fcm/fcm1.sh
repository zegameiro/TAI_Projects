#!/bin/bash

EXECUTABLE="../../../target/debug/fcm"

FILE="../../../data/sequences/sequence7.txt"
K=4
ALPHA=0.1

$EXECUTABLE $FILE -k $K -a $ALPHA