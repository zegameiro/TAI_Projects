import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import numpy as np

VISUALIZATIONS_DIR = "../visualizations/"

# Load your data
df = pd.read_csv("../tests/meta_results_release.csv")  # Or use pd.read_clipboard() if pasted

# Convert alpha to string for better axis labeling
df['alpha_str'] = df['alpha'].astype(str)

# --- 1. Line Plots ---
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

# --- 2. Heatmaps ---
pivot_metrics = {}
print("\nGenerating heatmaps...")
for metric in ['train_time', 'nrc_time', 'total_time']:
    print(f"\tCreating heatmap for {metric}...")
    pivot = df.pivot(index='alpha', columns='k', values=metric)
    pivot_metrics[metric] = pivot
    plt.figure(figsize=(12, 5))
    sns.heatmap(pivot, annot=True, fmt=".2f", cmap="YlGnBu")
    plt.title(f"{metric.replace('_', ' ').capitalize()} Heatmap")
    plt.xlabel("k")
    plt.ylabel("α")
    plt.tight_layout()
    plt.savefig(f"{VISUALIZATIONS_DIR}heatmap_{metric}.png")

# --- 3. Stacked Bar Chart (Time Breakdown) ---
print("\nGenerating stacked bar chart...")
plt.figure(figsize=(12, 6))
grouped = df[df['alpha'] == 0.01]
plt.bar(grouped['k'], grouped['train_time'], label='Train Time')
plt.bar(grouped['k'], grouped['nrc_time'], bottom=grouped['train_time'], label='NRC Time')
plt.title("Execution Time Breakdown (α = 0.01)")
plt.xlabel("k")
plt.ylabel("Time (s)")
plt.legend()
plt.tight_layout()
plt.savefig(f"{VISUALIZATIONS_DIR}stacked_bar_alpha_0.01.png")
