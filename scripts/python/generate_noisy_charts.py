import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

TESTS_DIR = "../../tests/noise/"
NOISES = [None, "white", "brown", "pink", "green"]
COMPRESSORS = ["gz", "bz2", "xz", "zstd", "fcm"]

def get_percent_correct_samples_matched(csv_path):
    try:
        df = pd.read_csv(csv_path)
        if df.empty:
            return 0

        correct_samples = df.apply(lambda row: row['SampleWithNoise'] == row['MostSimilarSample'], axis=1).sum()
        total_samples = df.shape[0]
        return (correct_samples / total_samples) * 100 if total_samples > 0 else 0
    except Exception as e:
        print(f"Error reading {csv_path}: {e}")
        return 0

def main():
    fig, ax = plt.subplots(figsize=(8, 5))
    bar_width = 0.15
    index = np.arange(len(NOISES))

    for i, compressor in enumerate(COMPRESSORS):
        values = []
        for noise in NOISES:
            csv_path = f"{TESTS_DIR}{compressor}/ncd_{noise}.csv"
            value = get_percent_correct_samples_matched(csv_path)
            if value is None:
                value = 0
            values.append(value)
        ax.bar(index + i * bar_width, values, bar_width, label=compressor)

    ax.set_xlabel('Noise type')
    ax.set_ylabel('Percent (%)')
    ax.set_title('Matching Noisy Samples with Original Samples as the most similar')
    ax.set_xticks(index + bar_width * (len(COMPRESSORS) - 1) / 2)
    noises_labels = [noise if noise is not None else "No Noise" for noise in NOISES]
    ax.set_xticklabels(noises_labels)
    ax.legend(title="Compressor")
    ax.set_ylim(0, 100)
    plt.tight_layout()
    plt.savefig(f"{TESTS_DIR}noisy_chart.png")
    print(f"Chart saved to {TESTS_DIR}noisy_chart.png")

if __name__ == "__main__":
    main()
