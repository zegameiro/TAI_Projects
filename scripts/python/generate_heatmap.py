import subprocess
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns

def run_metaclass(k, alpha):
    try:
        result = subprocess.run(
            ["../../target/debug/metaClass", "-d", "../../data/db.txt", "-s", "../../data/meta.txt", "-k", str(k), "-a", str(alpha), "-t", "20"],
            capture_output=True,
            text=True,
            check=True
        )
        return result.stdout
    except subprocess.CalledProcessError as e:
        print(f"Error running MetaClass with k={k}, alpha={alpha}")
        print(e.stderr)
        return ""

def extract_top20_nrc(output):
    """Extract a list of 20 NRC values from the output."""
    lines = output.splitlines()
    start_idx = None
    for i, line in enumerate(lines):
        if "Top 20 sequences:" in line:
            start_idx = i + 1
            break
    if start_idx is None:
        return [0]*20  # Return default if failed

    scores = []
    for line in lines[start_idx:start_idx + 20]:
        try:
            score = float(line.strip().split(":")[-1])
            scores.append(score)
        except:
            scores.append(0.0)
    while len(scores) < 20:
        scores.append(0.0)
    return scores

VISUALIZATIONS_DIR = "../../visualizations/"
# Heatmap 1: Varying k (fixed alpha)
k_values = [2, 4, 6, 8, 10, 12, 14, 16]
alpha_fixed = 0.01

top20_matrix_k = []
print(f"\nRunning MetaClass with fixed alpha = {alpha_fixed}")
for k in k_values:
    print(f"\tfor k = {k}")
    output = run_metaclass(k, alpha_fixed)
    top20 = extract_top20_nrc(output)
    top20_matrix_k.append(top20)

top20_matrix_k = np.array(top20_matrix_k).T  # Transpose: rows = rank, cols = k

plt.figure(figsize=(10, 8))
sns.heatmap(top20_matrix_k, annot=True, fmt=".3f", xticklabels=k_values, yticklabels=[f"#{i+1}" for i in range(20)], cmap="YlGnBu")
plt.title(f"Top 20 NRC Scores per k (α = {alpha_fixed})")
plt.xlabel("k")
plt.ylabel("Rank")
plt.tight_layout()
plt.savefig(VISUALIZATIONS_DIR + "heatmap_top20_k.png")

# Heatmap 2: Varying alpha (fixed k)
alpha_values = [0.001, 0.005, 0.01, 0.05, 0.1, 0.2, 0.3, 0.5]
k_fixed = 10

top20_matrix_alpha = []
print(f"\nRunning MetaClass with fixed k = {k_fixed}")
for alpha in alpha_values:
    print(f"\tfor alpha = {alpha}")
    output = run_metaclass(k_fixed, alpha)
    top20 = extract_top20_nrc(output)
    top20_matrix_alpha.append(top20)

top20_matrix_alpha = np.array(top20_matrix_alpha).T  # Transpose

plt.figure(figsize=(10, 8))
sns.heatmap(top20_matrix_alpha, annot=True, fmt=".3f", xticklabels=alpha_values, yticklabels=[f"#{i+1}" for i in range(20)], cmap="PuRd")
plt.title(f"Top 20 NRC Scores per α (k = {k_fixed})")
plt.xlabel("α")
plt.ylabel("Rank")
plt.tight_layout()
plt.savefig(VISUALIZATIONS_DIR + "heatmap_top20_alpha.png")
