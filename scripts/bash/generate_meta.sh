#!/bin/bash

OUTPUT_FILE="../../tests/meta_results_debug.csv"
# OUTPUT_FILE="tests/meta_results_release.csv"

echo "k,alpha,train_time,nrc_time,total_time" > $OUTPUT_FILE

# Função para extrair valores de tempo e converter para segundos
extract_time() {
    local time_str=$1
    
    # Tenta extrair o valor e converter para segundos
    if [[ $time_str =~ ([0-9]+\.[0-9]+)s ]]; then
        echo "${BASH_REMATCH[1]}"
    elif [[ $time_str =~ ([0-9]+\.[0-9]+)ms ]]; then
        echo "scale=6; ${BASH_REMATCH[1]}/1000" | bc
    elif [[ $time_str =~ ([0-9]+)ms ]]; then
        echo "scale=6; ${BASH_REMATCH[1]}/1000" | bc
    else
        echo "0"
    fi
}

alpha_values=(0.01 0.1 1)

for alpha in "${alpha_values[@]}"; do
    for k in {1..20}; do
        echo "Running with k=$k, alpha=$alpha"
        
        output=$(./target/debug/metaClass -s ../../data/meta.txt -d ../../data/db.txt -k $k -a $alpha)
        # output=$(./target/release/metaClass -s data/meta.txt -d data/db.txt -k $k -a $alpha)
        
        train_line=$(echo "$output" | grep "Time taken to train the model")
        nrc_line=$(echo "$output" | grep "Time taken to compute NRC scores")
        total_line=$(echo "$output" | grep "Total time taken")
        
        train_time=$(extract_time "$train_line")
        nrc_time=$(extract_time "$nrc_line")
        total_time=$(extract_time "$total_line")
        
        echo "$k,$alpha,$train_time,$nrc_time,$total_time" >> $OUTPUT_FILE
    done
done

echo "Tests completed. Results saved in $OUTPUT_FILE"