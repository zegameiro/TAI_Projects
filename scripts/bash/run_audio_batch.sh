#!/bin/bash

MUSIC_DIR="./music" 
AUDIO_EXE="./target/release/audio"
SEGMENT_MS=1000
TOP_N=10
TOP_K=15

STARTS=(0 20000 40000)
ENDS=(20000 60000 100000)
COMPRESSORS=("gz" "bz2" "xz" "zstd" "lzma")

CSV_OUT="audio_batch_results.csv"
echo "sample,start,end,compressor,correct" > "$CSV_OUT"

for SONG in "$MUSIC_DIR"/*.wav; do
    BASENAME=$(basename "$SONG")
    for IDX in "${!STARTS[@]}"; do
        START=${STARTS[$IDX]}
        END=${ENDS[$IDX]}
        for COMP in "${COMPRESSORS[@]}"; do
            echo "Running: $SONG | start=$START | end=$END | compressor=$COMP"
            OUTPUT=$("$AUDIO_EXE" \
                -s "$SONG" \
                -d "$MUSIC_DIR" \
                -l "$SEGMENT_MS" \
                -n "$TOP_N" \
                -k "$TOP_K" \
                -c "$COMP" \
                --start "$START" \
                --end "$END")
            echo "$OUTPUT"
            # Check if the sample filename appears in the top results
            # Look for the first "Top" section and check if BASENAME is present in the next $TOP_K lines
            CORRECT=0
            TOP_LINES=$(echo "$OUTPUT" | awk "/Top $TOP_K closest music files \(dominant frequencies\):/{flag=1;next}/^$/{flag=0}flag" | head -n "$TOP_K")
            if echo "$TOP_LINES" | grep -q "$BASENAME"; then
                CORRECT=1
            fi
            echo "$BASENAME,$START,$END,$COMP,$CORRECT" >> "$CSV_OUT"
            echo "---------------------------------------------"
        done
    done
done
