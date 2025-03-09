#!/bin/bash

EXECUTABLE="target/debug/fcm"

FILE="sequences/sequence1.txt"
K=3
ALPHA=10

$EXECUTABLE $FILE -k $K -a $ALPHA