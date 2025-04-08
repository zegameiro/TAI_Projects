#!/bin/bash

mkdir -p data

output_db="data/db_test.txt"
> "$output_db"

num_db_samples=50

# db_test.txt
for i in $(seq 1 $num_db_samples); do
    # 1000 - 10000
    len=$((RANDOM % 9001 + 1000))

    sequence=$(gto_genomic_gen_random_dna -s "$i" -n "$len")

    echo "@seq_$i" >> "$output_db"

    echo "$sequence" | fold -w 75 >> "$output_db"
done


# meta_n.txt
max=113
for n in $(seq 1 5); do
    output_mutate="data/meta_${n}.txt"
    mkdir -p "$(dirname "$output_mutate")"
    > "$output_mutate"


    for s in $(seq 1 $max); do

        seq_id=$((RANDOM % n + 1))

        seq=$(awk -v id="@seq_$seq_id" '
            $0 ~ id {flag=1; next}
            /^@/ {flag=0}
            flag {printf "%s", $0}
        ' data/db_test.txt)

        sequence=$(echo "$seq" | cut -c1-151)


        printf "%s\n" "$sequence" | gto_genomic_dna_mutate -s "$s" -m 0.5 | tr -d '\n' >> "$output_mutate"
        echo "" >> "$output_mutate"

        percent=$(awk -v n="$n" -v s="$s" -v max="$max" '
            BEGIN { printf "%.2f", ((n - 1 + s / max) / 5) * 100 }
        ')
        echo -ne "$percent% \r"
    done
done

