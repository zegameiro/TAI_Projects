#!/bin/bash

cargo build --release

EXECUTABLE="target/release/metaClass"

DB_FILE="data/db.txt"
META_FILE="data/meta.txt"

K=10
ALPHA=0.01

echo "Running metaClass..."
$EXECUTABLE -d "$DB_FILE" -s "$META_FILE" -k "$K" -a "$ALPHA"
