import json
import matplotlib.pyplot as plt
import seaborn as sns
import numpy as np
import pandas as pd

# Load the JSON data
with open('../../comparative_nrc_results.json', 'r') as f:
    data = json.load(f)

# Extract all unique sequence names, keeping original full names
sequence_names = []
for entry in data:
    # Keep the full original name
    base_name = entry['base_sequence']
    if base_name not in sequence_names:
        sequence_names.append(base_name)

# Create a DataFrame to store the NRC scores
nrc_matrix = pd.DataFrame(index=sequence_names, columns=sequence_names)

# Fill the matrix with NRC scores
for entry in data:
    # Use full original names
    base_name = entry['base_sequence']
    
    for match in entry['matches']:
        target_name = match['target_name']
        nrc_score = match['nrc_score']
        nrc_matrix.loc[base_name, target_name] = nrc_score

# Fill any NaN values with a default
nrc_matrix = nrc_matrix.fillna(0)

# Create a larger figure to accommodate the longer names
plt.figure(figsize=(16, 14))
ax = sns.heatmap(nrc_matrix, annot=True, fmt=".3f", cmap="coolwarm", 
                linewidths=0.5, cbar_kws={"shrink": 0.8})

# Customize the plot
plt.title("NRC Scores Comparison Matrix", fontsize=16)

# Rotate x-axis labels for better readability of long names
plt.xticks(rotation=45, ha='right', fontsize=9)
plt.yticks(rotation=0, fontsize=9)

# Save the figure
plt.savefig("../../visualizations/nrc_comparison_heatmap.png", dpi=300, bbox_inches='tight')