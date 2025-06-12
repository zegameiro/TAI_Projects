#!/bin/bash

output_file="image_results/summary.md"
mkdir -p image_results
> "$output_file"

# Define compressors to test
compressors=("gz" "bz2" "xz" "zstd" "lzma")
declare -A totals  # associative array to hold total counts

# Initialize totals
for comp in "${compressors[@]}"; do
    totals["$comp"]=0
done

# Write Markdown table header
{
    printf "| Image |"
    for c in "${compressors[@]}"; do
        printf " %s |" "$c"
    done
    echo
    printf "|--------|"
    for _ in "${compressors[@]}"; do
        printf "%s" "------|"
    done
    echo
} >> "$output_file"

# Loop through images
for i in {1..41}; do
    image_file="../../../data/images/$((10 * (i - 1) + 1))_${i}.jpg"
    pattern="_${i}"
    image_name=$(basename "$image_file")

    printf "| %s |" "$image_name" >> "$output_file"

    for comp in "${compressors[@]}"; do
        result=$("../../../target/release/image_ncd" -i "$image_file" -d "../../../data/images" -c "$comp" -l 128)
        count=$(echo "$result" | grep -c "$pattern")
        totals["$comp"]=$((totals["$comp"] + count))
        printf " %s |" "$count" >> "$output_file"
    done

    echo >> "$output_file"
done

# Append totals row
{
    printf "| **Total** |"
    for comp in "${compressors[@]}"; do
        printf " %s |" "${totals[$comp]}"
    done
    echo
} >> "$output_file"
