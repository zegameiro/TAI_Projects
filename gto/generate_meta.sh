#!/bin/bash

output_random="data/meta_random.txt"
mkdir -p "$(dirname "$output_random")"
> "$output_random"

output_mutate="data/meta_mutate.txt"
mkdir -p "$(dirname "$output_mutate")"
> "$output_mutate"

max=11310

for s in $(seq 1 $max); do

    sequence=$(gto_genomic_gen_random_dna -s "$s" -n 151)

    echo "$sequence" >> "$output_random"

    printf "%s\n" "$sequence" | gto_genomic_dna_mutate -s "$s" -m 0.5 | tr -d '\n' >> "$output_mutate"
    echo "" >> "$output_mutate"

    percent=$(awk "BEGIN {printf \"%.2f\", ($s / $max) * 100}")
    echo -ne "$percent% \r"
done

