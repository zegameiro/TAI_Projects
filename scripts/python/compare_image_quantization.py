import matplotlib.pyplot as plt
import numpy as np

# Data
quantization = [4, 8, 16, 32, 64, 128, 256]
images = [2, 3, 4, 8, 3, 8, 10]

# Create the figure and axis
plt.figure(figsize=(10, 6))

# Plot line chart
plt.plot(quantization, images, marker='o', linestyle='-', linewidth=2, markersize=8)

# Add labels and title
plt.xlabel('Quantization Levels', fontsize=12)
plt.ylabel('Number of Images', fontsize=12)
plt.title('Number of Images per Quantization Level', fontsize=14)

# Set x-axis to use log scale since quantization values increase exponentially
plt.xscale('log', base=2)
plt.xticks(quantization, labels=[str(q) for q in quantization])

# Add grid for better readability
plt.grid(True, linestyle='--', alpha=0.7)

# Add data labels
for x, y in zip(quantization, images):
    plt.text(x, y+0.3, str(y), ha='center', va='bottom')

# Set y-axis limits to start from 0 and have some padding at the top
plt.ylim(0, max(images) + 1.5)

# Improve layout
plt.tight_layout()

# Save the figure
plt.savefig('../../visualizations/quantization_vs_images.png', dpi=300)