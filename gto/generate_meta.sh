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


# meta_m_XX.txt (XX being the mutation percentage)
max=113
mutation_percentages=(25 50 75 100) # Array of mutation percentages

for m_percent in "${mutation_percentages[@]}"; do
    output_mutate="data/meta_${m_percent}.txt"
    mkdir -p "$(dirname "$output_mutate")"
    > "$output_mutate"

    echo "Generating mutated sequences with ${m_percent}% mutation rate..."

    for s in $(seq 1 $max); do
        # We will always select from the full database for mutation
        seq_id=$((RANDOM % num_db_samples + 1))

        seq_header=$(awk -v id="@seq_$seq_id" '
            $0 ~ id {print; exit}
        ' data/db_test.txt)

        seq_full=$(awk -v id="@seq_$seq_id" '
            $0 ~ id {flag=1; next}
            /^@/ {flag=0}
            flag {printf "%s", $0}
        ' data/db_test.txt)

        sequence=$(echo "$seq_full" | cut -c1-151)

        echo "Original Sequence (from $seq_header): $sequence" >> "$output_mutate"

        mutated_sequence=$(printf "%s\n" "$sequence" | gto_genomic_dna_mutate -s "$s" -m "$m_percent" | tr -d '\n')
        echo "Mutated Sequence: $mutated_sequence" >> "$output_mutate"
        echo "" >> "$output_mutate"

        percent=$(awk -v m="$m_percent" -v s="$s" -v max="$max" '
            BEGIN { printf "%.2f", (s / max) * (m / 100) * 100 }
        ')
        echo -ne "Mutation: $m_percent%, Progress: $percent% \r"
    done
    echo "" # Add a newline after each mutation percentage is done
    echo "Finished generating mutated sequences with ${m_percent}% mutation rate."
    echo ""
done