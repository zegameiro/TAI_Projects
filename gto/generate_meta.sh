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

# Meta files with varying mutation rates (0-100%) for fixed random sequences
echo "--- Generating meta files with varying mutation rates (0-100%) ---"
num_meta_files=5
num_random_sequences_part2=2 # Number of random sequences to choose
num_generated_sequences_part2=10 # Number of new generated sequences

# Choose 2 random sequences once for all meta files in this part
fixed_random_sequence_ids=$(shuf -i 1-"$num_db_samples" -n "$num_random_sequences_part2")
fixed_random_sequence_array=($fixed_random_sequence_ids)

mutation_percentages_part2=$(seq 0 25 100) # Mutation percentages from 0 to 100 with 25% step

for k in $(seq 1 "$num_meta_files"); do
    m_percent_part2="${mutation_percentages_part2[$((k - 1))]}" # Get the corresponding percentage
    output_mutate_part2="data/meta_varying_mutation_${k}.txt"
    mkdir -p "$(dirname "$output_mutate_part2")"
    > "$output_mutate_part2"

    echo "Generating meta file: $output_mutate_part2 with ${m_percent_part2}% mutation rate..."

    for i in $(seq 0 $((num_random_sequences_part2 - 1))); do
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

        echo "Original Sequence (from $seq_header): $original_sequence" >> "$output_mutate_part2"

        # Mutate the original sequence
        new_seed_mutate_part2=$((RANDOM + 10#$(date +%N) + k * 100 + i ))
        mutated_sequence_part2=$(echo "$original_sequence" | gto_genomic_dna_mutate -s "$new_seed_mutate_part2" -m "$m_percent_part2" | tr -d '\n')
        echo "Mutated Sequence (at ${m_percent_part2}%): $mutated_sequence_part2" >> "$output_mutate_part2"

        # Generate new random sequences
        for j in $(seq 1 "$num_generated_sequences_part2"); do
            new_seed_rand_part2=$((RANDOM + 10#$(date +%N) + k * 1000 + i * 100 + j ))
            random_generated_sequence_part2=$(gto_genomic_gen_random_dna -s "$new_seed_rand_part2" -n 151)
            echo "Generated Sequence #$j (based on $seq_header): $random_generated_sequence_part2" >> "$output_mutate_part2"
        done
        echo "" >> "$output_mutate_part2" # Separator
    done

    echo "Finished generating meta file: $output_mutate_part2"
    echo ""
done