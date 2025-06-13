import subprocess
import re
import csv
from pathlib import Path
import matplotlib.pyplot as plt

EXECUTABLE = "../../target/release/audio"
MUSIC_DIR = "../../music/"
NOISY_DIR = "../../noisy/"
TESTS_DIR = "../../tests/similar/"

samples = [
    "a_cabritinha_quim_barreiros",
    "hitchin_a_ride_green_day",
    "levitating_dua_lipa"
]

def extract_top4_samples(sample):
    sample_path = Path(NOISY_DIR) / f"{sample}.wav"
    cmd = [EXECUTABLE, "-s", str(sample_path), "-d", MUSIC_DIR]
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        output = result.stdout
        lines = output.splitlines()
        top4 = []
        for i, line in enumerate(lines):
            if "Top 4 closest music files (dominant frequencies):" in line:
                for j in range(1, 5):
                    if i + j < len(lines):
                        match = re.search(r"\d+\.\s+([^\s]+)\s+NCD:\s*([0-9.]+)", lines[i + j])
                        if match:
                            sample_name = match.group(1).replace(".wav", "")
                            ncd_value = float(match.group(2))
                            top4.append((sample_name, ncd_value))
                break
        return top4
    except subprocess.CalledProcessError as e:
        print(f"Erro ao processar {sample_path}: {e}")
    return []

def plot_tables(sample_results):
    out_dir = Path(TESTS_DIR)
    out_dir.mkdir(parents=True, exist_ok=True)
    for sample, top4 in sample_results.items():
        fig, ax = plt.subplots(figsize=(7, 3))
        cell_text = [[rank, sim_sample, f"{ncd:.4f}"] for rank, (sim_sample, ncd) in enumerate(top4, 1)]
        col_labels = ["Rank", "MostSimilarSample", "NCD"]
        ax.axis('off')
        table = ax.table(cellText=cell_text, colLabels=col_labels, loc='center')
        table.auto_set_font_size(False)
        table.set_fontsize(10)
        table.scale(1.2, 1.2)
        for key, cell in table.get_celld().items():
            col = key[1]
            if col == 1:
                cell.set_width(0.7)
            else:
                cell.set_width(0.2)
        ax.set_title(sample)
        plt.tight_layout()
        out_path = out_dir / f"top4_similar_{sample}.png"
        plt.savefig(out_path)
        print(f"Table saved at {out_path}")
        plt.close()

def main():
    sample_results = {}
    for sample in samples:
        print(f"Extracting top 4 similars samples to {sample}")
        top4 = extract_top4_samples(sample)
        sample_results[sample] = top4
        for rank, (similar_sample, ncd) in enumerate(top4, 1):
            print(f"\t{rank}. {similar_sample} (NCD: {ncd:.4f})")
    if sample_results:
        plot_tables(sample_results)

if __name__ == "__main__":
    main()
