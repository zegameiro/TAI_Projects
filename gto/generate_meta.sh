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
num_meta_sequences=10 # Number of new random sequences to generate per meta file
mutation_percentages=(25 50 75 100) # Array of mutation percentages

for m_percent in "${mutation_percentages[@]}"; do
    output_mutate="data/meta_${m_percent}.txt"
    mkdir -p "$(dirname "$output_mutate")"
    > "$output_mutate"

    echo "Generating meta file with ${m_percent}% mutation rate..."

    # Choose 5 random sequences from db_test.txt
    random_sequence_ids=$(shuf -i 1-"$num_db_samples" -n 5)

    for seq_id in $random_sequence_ids; do
        seq_header=$(awk -v id="@seq_$seq_id" '
            $0 ~ id {print; exit}
        ' data/db_test.txt)

        seq_full=$(awk -v id="@seq_$seq_id" '
            $0 ~ id {flag=1; next}
            /^@/ {flag=0}
            flag {printf "%s", $0}
        ' data/db_test.txt)

        original_sequence=$(echo "$seq_full" | cut -c1-151)

        echo "Original Sequence (from $seq_header): $original_sequence" >> "$output_mutate"

        # Generate num_meta_sequences random sequences based on the chosen original
        for j in $(seq 1 "$num_meta_sequences"); do
            # Use a different seed for each generated sequence
            new_seed=$((RANDOM + $(date +%N) ))
            random_mutated_sequence=$(gto_genomic_gen_random_dna -s "$new_seed" -n 151)
            echo "Generated Sequence #$j (based on $seq_header): $random_mutated_sequence" >> "$output_mutate"

            # Optionally, you could still apply a mutation here if needed, e.g.:
            # mutated_sequence=$(echo "$random_mutated_sequence" | gto_genomic_dna_mutate -s "$new_seed" -m "$m_percent" | tr -d '\n')
            # echo "Mutated Generated Sequence #$j: $mutated_sequence" >> "$output_mutate"
        done
        echo "" >> "$output_mutate" # Add a separator after each original sequence's generated sequences
    done

    echo "Finished generating meta file: $output_mutate"
    echo ""
done