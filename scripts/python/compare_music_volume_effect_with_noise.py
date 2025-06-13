import subprocess
import re
import matplotlib.pyplot as plt
from pathlib import Path

EXECUTABLE = "../../target/release/audio"
MUSIC_DIR = "../../music/"
NOISY_DIR = "../../noisy/"
TESTS_DIR = "../../tests/noise/"

samples = [
    "hitchin_a_ride_green_day",
    "toxicity_system_of_a_down",
    "the_still_sea_the_sweetgreens",
    "the_four_seasons_baroque_festival_orchestra"
]
noises = ["white", "brown", "pink", "green"]

def extract_ncd_max(sample, noise):
    sample_path = Path(NOISY_DIR) / f"{sample}_{noise}.wav"
    cmd = [EXECUTABLE, "-s", str(sample_path), "-d", MUSIC_DIR]
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        output = result.stdout
        lines = output.splitlines()
        for i, line in enumerate(lines):
            if f"Processing file: {sample}.wav" in line:
                # Procura na linha seguinte o NCD score (max_freqs)
                if i + 1 < len(lines):
                    match = re.search(r"NCD score \(max_freqs\): ([0-9.]+)", lines[i + 1])
                    if match:
                        return float(match.group(1))
        return None
    except subprocess.CalledProcessError as e:
        print(f"Erro ao processar {sample_path}: {e}")
    return None

def main():
    ncd_results = {sample: [] for sample in samples}

    for sample in samples:
        print(f"Processing {sample}")
        for noise in noises:
            ncd_max = extract_ncd_max(sample, noise)
            ncd_results[sample].append(ncd_max)
            print(f"\tNoise: {noise} -> NCD: {ncd_max}")

    plt.figure(figsize=(8, 5))
    colors = ['tab:red', 'tab:orange', 'tab:green', 'tab:blue']
    for idx, (sample, ncds) in enumerate(ncd_results.items()):
        plt.plot(noises, ncds, marker='o', color=colors[idx], label=sample)
        plt.scatter(noises, ncds, color=colors[idx], s=80)
    plt.xlabel("Noise type")
    plt.ylabel("NCD score (max_freqs)")
    plt.title("NCD score for each sample with different noises compared to original")
    plt.legend()
    plt.tight_layout()
    out_path = Path(TESTS_DIR) / "ncd_noise_effect.png"
    plt.savefig(out_path)
    print(f"Graph saved at {out_path}")
    # plt.show()

if __name__ == "__main__":
    main()
