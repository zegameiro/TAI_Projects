import subprocess
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns
import os

def run_metaclass(meta_file, k=20, alpha=0.01):
    try:
        result = subprocess.run(
            ["../target/debug/metaClass", "-d", "../data/db_test.txt", "-s", meta_file, "-k", str(k), "-a", str(alpha), "-t", "20"],
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

# Mutation-level files
mutation_files = {
    "0%": "../data/meta_varying_mutation_0.txt",
    "25%": "../data/meta_varying_mutation_5.txt",
    "50%": "../data/meta_varying_mutation_10.txt",
    "75%": "../data/meta_varying_mutation_15.txt",
    "100%": "../data/meta_varying_mutation_100.txt"
}

# Containers for all results
all_scores = {}  # For heatmaps
all_names = {}   # For sequence names
avg_scores = {}  # For line plot

# Run analysis for each file
for label, filename in mutation_files.items():
    print(f"Processing {label} mutation: {filename}")
    if not os.path.exists(filename):
        print(f"❌ File not found: {filename}")
        continue

    output = run_metaclass(filename)
    top20, seq_names = extract_top20_nrc(output)
    all_scores[label] = top20
    all_names[label] = seq_names
    avg_scores[label] = np.mean(top20)

    # Create labels that combine rank and sequence name
    y_labels = [f"#{i+1}: {name}" for i, name in enumerate(seq_names)]
    
    # Plot heatmap for this mutation level
    plt.figure(figsize=(10, 8))
    sns.heatmap(
        np.array(top20).reshape(-1, 1),
        annot=True, fmt=".3f",
        yticklabels=y_labels,
        xticklabels=[label],
        cmap="coolwarm"
    )
    plt.title(f"Top 20 NRC Scores ({label} Mutation)")
    plt.ylabel("Rank: Sequence")
    plt.tight_layout()
    plt.savefig(f"heatmap_nrc_{label.replace('%', '')}.png")

# Line plot of average NRC across mutation levels
sorted_labels = sorted(avg_scores.keys(), key=lambda x: int(x.replace('%', '')))
avg_values = [avg_scores[label] for label in sorted_labels]

plt.figure(figsize=(8, 5))
plt.plot(sorted_labels, avg_values, marker='o', linewidth=2)
plt.title("Average NRC vs Mutation Level")
plt.xlabel("Mutation Level")
plt.ylabel("Average NRC")
plt.grid(True)
plt.tight_layout()
plt.savefig("avg_nrc_vs_mutation.png")