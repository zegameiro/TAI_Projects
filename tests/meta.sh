#!/bin/bash

OUTPUT_FILE="tests/meta_results.csv"

echo "k,alpha,train_time,nrc_time,total_time" > $OUTPUT_FILE

alpha_values=(0.01 0.1 1)

for alpha in "${alpha_values[@]}"; do
    for k in {1..100}; do
        echo "Running with k=$k, alpha=$alpha"
        
        output=$(./target/debug/metaClass -s data/meta.txt -d data/db.txt -k $k -a $alpha)
        
        train_time=$(echo "$output" | grep "Time taken to train the model" | sed -E 's/.*: ([0-9]+\.[0-9]+)s/\1/' | sed -E 's/.*: ([0-9]+)ms/\1/' | sed -E 's/.*: ([0-9]+)µs/0.\1/')
        nrc_time=$(echo "$output" | grep "Time taken to compute NRC scores" | sed -E 's/.*: ([0-9]+\.[0-9]+)s/\1/' | sed -E 's/.*: ([0-9]+)ms/\1/' | sed -E 's/.*: ([0-9]+)µs/0.\1/')
        total_time=$(echo "$output" | grep "Total time taken" | sed -E 's/.*: ([0-9]+\.[0-9]+)s/\1/' | sed -E 's/.*: ([0-9]+)ms/\1/' | sed -E 's/.*: ([0-9]+)µs/0.\1/')
        
        echo "$k,$alpha,$train_time,$nrc_time,$total_time" >> $OUTPUT_FILE
    done
done

echo "Tests completed. Results saved in $OUTPUT_FILE"