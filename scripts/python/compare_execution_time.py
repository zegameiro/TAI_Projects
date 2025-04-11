import pandas as pd
import matplotlib.pyplot as plt

VISUALIZATIONS_DIR = "../../visualizations/"

df = pd.read_csv("../../tests/meta_results_release.csv")

df['alpha_str'] = df['alpha'].astype(str)

plt.figure(figsize=(12, 6))
print("\nGenerating line plots...")
for metric in ['train_time', 'nrc_time', 'total_time']:
    print(f"\tPlotting {metric}...")
    plt.figure(figsize=(10, 5))
    for alpha in sorted(df['alpha'].unique()):
        subset = df[df['alpha'] == alpha]
        plt.plot(subset['k'], subset[metric], marker='o', label=f'α = {alpha}')
    plt.title(f'{metric.replace("_", " ").capitalize()} vs k')
    plt.xlabel("k")
    plt.ylabel("Time (s)")
    plt.legend()
    plt.grid(True)
    plt.tight_layout()
    plt.savefig(f"{VISUALIZATIONS_DIR}lineplot_{metric}.png")


