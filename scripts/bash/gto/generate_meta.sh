#!/bin/bash

mkdir -p data

output_db="../../../data/generated/db_test.txt"
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

# Define mutation percentages properly as an array
mutation_percentages=(0 1 5 10 15 20 25)
# Set number of meta files based on mutation percentages length
num_meta_files=${#mutation_percentages[@]}

num_fixed_sequences=5 # Number of fixed random sequences to choose
num_generated_sequences=30 # Number of new generated sequences per fixed sequence

# Choose 5 random sequences once for all meta files in this part
fixed_random_sequence_ids=$(shuf -i 1-"$num_db_samples" -n "$num_fixed_sequences")
fixed_random_sequence_array=($fixed_random_sequence_ids)

# Get all original sequences first (do this only once)
echo "Getting original sequences and generating random sequences..."
declare -a original_sequences
declare -a seq_headers
declare -a random_sequences

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
    
    original_sequences[$i]="$seq_full"
    seq_headers[$i]="$seq_header"
    
    # Generate random sequences once (for all mutation files)
    for j in $(seq 1 "$num_generated_sequences"); do
        rand_seq_index=$((i * num_generated_sequences + j - 1))
        new_seed_rand=$((RANDOM + 10#$(date +%N) + i * 100 + j))
        random_sequences[$rand_seq_index]=$(gto_genomic_gen_random_dna -s "$new_seed_rand" -n 151)
    done
done

# Now create each meta file with different mutation rates
for k in $(seq 0 $((num_meta_files - 1))); do
    # Access array element properly using index k
    m_percent_int=${mutation_percentages[$k]}
    m_percent_float=$(echo "scale=2; $m_percent_int / 100" | bc)
    output_mutate="../../../data/generated/meta_varying_mutation_${m_percent_int}percent.txt"
    mkdir -p "$(dirname "$output_mutate")"
    > "$output_mutate"

    echo "Generating meta file: $output_mutate with ${m_percent_int}% mutation rate (float: $m_percent_float)..."

    # For each original sequence
    for i in $(seq 0 $((num_fixed_sequences - 1))); do
        original_sequence="${original_sequences[$i]}"
        
        # Mutate the original sequence with current percentage
        new_seed_mutate=$((RANDOM + 10#$(date +%N) + k * 100 + i))
        mutated_sequence=$(echo "$original_sequence" | gto_genomic_dna_mutate -s "$new_seed_mutate" -m "$m_percent_float" | tr -d '\n')
        
        # Write just the sequence without prefix
        echo "$mutated_sequence" | fold -w 121 >> "$output_mutate"
        echo "" >> "$output_mutate"
        
        # Write all associated random sequences without prefixes
        for j in $(seq 1 "$num_generated_sequences"); do
            rand_seq_index=$((i * num_generated_sequences + j - 1))
            echo "${random_sequences[$rand_seq_index]}" | fold -w 121 >> "$output_mutate"
            echo "" >> "$output_mutate"
        done
    done

    echo "Finished generating meta file: $output_mutate"
    echo ""
done

# Print summary of sequences used
echo "=== Sequences used in this experiment ==="
for i in $(seq 0 $((num_fixed_sequences - 1))); do
    echo "Original sequence ${i+1}: ${seq_headers[$i]}"
done

echo "Script finished."