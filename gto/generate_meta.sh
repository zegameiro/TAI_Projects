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

# Part 2: Meta files with varying mutation rates (0-100%) for fixed random sequences
echo "--- Generating meta files with varying mutation rates (0-100%) for fixed sequences ---"
num_meta_files=5
num_fixed_sequences=5 # Number of fixed random sequences to choose
num_generated_sequences=30 # Number of new generated sequences per fixed sequence

# Choose 5 random sequences once for all meta files in this part
fixed_random_sequence_ids=$(shuf -i 1-"$num_db_samples" -n "$num_fixed_sequences")
fixed_random_sequence_array=($fixed_random_sequence_ids)

# Define mutation percentages properly as an array
mutation_percentages=(0 25 50 75 100)

for k in $(seq 1 "$num_meta_files"); do
    # Access array element properly using index k-1
    m_percent_int=${mutation_percentages[$((k - 1))]}
    m_percent_float=$(echo "scale=2; $m_percent_int / 100" | bc)
    output_mutate="data/meta_varying_mutation_${k}.txt"
    mkdir -p "$(dirname "$output_mutate")"
    > "$output_mutate"

    echo "Generating meta file: $output_mutate with ${m_percent_int}% mutation rate (float: $m_percent_float)..."

    for i in $(seq 0 $((num_fixed_sequences - 1))); do
        seq_id="${fixed_random_sequence_array[$i]}"

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

        # Mutate the original sequence
        new_seed_mutate=$((RANDOM + 10#$(date +%N) + k * 100 + i ))
        mutated_sequence=$(echo "$original_sequence" | gto_genomic_dna_mutate -s "$new_seed_mutate" -m "$m_percent_float" | tr -d '\n')
        echo "Mutated Sequence (at ${m_percent_int}%): $mutated_sequence" >> "$output_mutate"

        # Generate new random sequences
        for j in $(seq 1 "$num_generated_sequences"); do
            new_seed_rand=$((RANDOM + 10#$(date +%N) + k * 1000 + i * 100 + j ))
            random_generated_sequence=$(gto_genomic_gen_random_dna -s "$new_seed_rand" -n 151)
            echo "Generated Sequence #$j (based on $seq_header): $random_generated_sequence" >> "$output_mutate"
        done
        echo "" >> "$output_mutate" # Separator
    done

    echo "Finished generating meta file: $output_mutate"
    echo ""
done

echo "Script finished."