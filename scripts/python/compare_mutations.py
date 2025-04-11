import subprocess
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns
import os
import pandas as pd

def run_metaclass(meta_file, k=10, alpha=0.01):
    try:
        result = subprocess.run(
            ["../../target/debug/metaClass", "-d", "../../data/db_test.txt", "-s", meta_file, "-k", str(k), "-a", str(alpha), "-t", "20"],
            capture_output=True,
            text=True,
            check=True
        )
        return result.stdout
    except subprocess.CalledProcessError as e:
        print(f"Error running MetaClass on {meta_file}")
        print(e.stderr)
        return ""

def extract_top20_nrc(output):
    """Extract list of 20 NRC scores and sequence names from output."""
    lines = output.splitlines()
    start_idx = None
    for i, line in enumerate(lines):
        if "Top 20 sequences:" in line:
            start_idx = i + 1
            break
    if start_idx is None:
        return [0]*20, ["Unknown"]*20

    scores = []
    seq_names = []
    for line in lines[start_idx:start_idx + 20]:
        try:
            parts = line.strip().split(":")
            score = float(parts[-1])
            # Extract sequence name (everything before the last colon)
            seq_name = ":".join(parts[:-1]).strip()
            # Make name more compact for display
            if "Generated Sequence #" in seq_name:
                seq_name = f"Gen#{seq_name.split('#')[1].split(' ')[0]}"
            elif "Mutated Sequence" in seq_name:
                seq_name = "Mutated"
            
            scores.append(score)
            seq_names.append(seq_name)
        except:
            scores.append(0.0)
            seq_names.append("Error")
    
    # Pad if needed
    while len(scores) < 20:
        scores.append(0.0)
        seq_names.append("N/A")
        
    return scores, seq_names

VISUALIZATIONS_DIR = "../../visualizations/"

# Mutation-level files
mutation_files = {
    "0%": "../../data/meta_varying_mutation_0percent.txt",
    "1%": "../../data/meta_varying_mutation_1percent.txt",
    "5%": "../../data/meta_varying_mutation_5percent.txt",
    "10%": "../../data/meta_varying_mutation_10percent.txt",
    "15%": "../../data/meta_varying_mutation_15percent.txt",
    "20%": "../../data/meta_varying_mutation_20percent.txt",
    "25%": "../../data/meta_varying_mutation_25percent.txt"
}

# Containers for all results
all_scores = {}  # For heatmaps
all_names = {}   # For sequence names
avg_scores = {}  # For line plot

# Run analysis for each file
for label, filename in mutation_files.items():
    print(f"Processing {label} mutation: {filename}")
    if not os.path.exists(filename):
        print(f"File not found: {filename}")
        continue

    output = run_metaclass(filename)
    top20, seq_names = extract_top20_nrc(output)
    all_scores[label] = top20
    all_names[label] = seq_names
    avg_scores[label] = np.mean(top20)

# Create sorted list of mutation labels
sorted_labels = sorted(all_scores.keys(), key=lambda x: int(x.replace('%', '')))

# Create a DataFrame for the combined heatmap (scores only)
score_matrix = np.zeros((20, len(sorted_labels)))
for i, label in enumerate(sorted_labels):
    if label in all_scores:
        score_matrix[:, i] = all_scores[label]

# Create a DataFrame for the visualization
df_scores = pd.DataFrame(score_matrix, columns=sorted_labels)

# Create a larger figure to accommodate the combined heatmap with annotations
plt.figure(figsize=(20, 15))

# Create the heatmap with score values
ax = sns.heatmap(df_scores, cmap="coolwarm", annot=False, fmt=".3f", linewidths=0.5)

# Add text annotations for both score and sequence name
for i in range(len(df_scores.index)):
    for j in range(len(df_scores.columns)):
        label = sorted_labels[j]
        if label in all_scores and label in all_names:
            score = all_scores[label][i]
            name = all_names[label][i]
            text = f"{score:.3f}\n{name}"
            ax.text(j + 0.5, i + 0.5, text, 
                   ha="center", va="center", fontsize=9,
                   color="white" if score > df_scores.values.mean() else "black")

# Set labels and title
plt.title("NRC Scores vs Mutation Rate for Top 20 Sequences", fontsize=16)
plt.ylabel("Rank (1-20)", fontsize=14)
plt.xlabel("Mutation Rate", fontsize=14)

# Set y-tick labels to show rank
plt.yticks(np.arange(20) + 0.5, [f"#{i+1}" for i in range(20)], fontsize=12)
plt.xticks(np.arange(len(sorted_labels)) + 0.5, sorted_labels, fontsize=12)

plt.tight_layout()
plt.savefig(f"{VISUALIZATIONS_DIR}combined_heatmap.png", dpi=300)

# Line plot of average NRC across mutation levels
avg_values = [avg_scores[label] for label in sorted_labels]

plt.figure(figsize=(8, 5))
plt.plot(sorted_labels, avg_values, marker='o', linewidth=2)
plt.title("Average NRC vs Mutation Level")
plt.xlabel("Mutation Level")
plt.ylabel("Average NRC")
plt.grid(True)
plt.tight_layout()
plt.savefig(f"{VISUALIZATIONS_DIR}avg_nrc_vs_mutation.png")