import subprocess
import re
import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt

DATA_PATH = "../data/"

files = {
    "real": "meta.txt",
    "mutated": "meta_mutate.txt"
}

# @gi|xxx|ref|...: 0.997332
score_pattern = re.compile(r": ([0-9]+\.[0-9]+)$")

scores = {}

for n in range(6):
    if n == 0:
        # original
        meta = "meta.txt"
        db = "db.txt"
        label = "original"
    else:
        # mutated
        meta = f"meta_{n}.txt"
        db = "db_test.txt"
        label = f"mutated w/{n} samples"

    meta_file = DATA_PATH + meta
    db_file = DATA_PATH + db

    result = subprocess.run(
        ["./../target/release/metaClass", "-s", meta_file, "-d", db_file, "-k", "10"],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )

    output_lines = result.stdout.splitlines()
    current_scores = []
    found_top = False
    for line in output_lines:
        if "Top 20 sequences" in line:
            found_top = True
            continue
        if found_top:
            match = score_pattern.search(line)
            if match:
                score = float(match.group(1))
                current_scores.append(score)
        if len(current_scores) == 20:
            break

    scores[label] = current_scores

df = pd.DataFrame(scores)

sns.heatmap(df.T, cmap="viridis", annot=False, fmt=".3f", cbar=True)
plt.title("Top 20 Sequences by Meta File")
plt.xlabel("Rank")
plt.ylabel("Type")
plt.tight_layout()
plt.savefig("heatmap.png", dpi=300)