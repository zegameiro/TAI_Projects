import subprocess
import re
import csv
from pathlib import Path

EXECUTABLE = "../../target/release/audio"
MUSIC_DIR = "../../music/"
NOISY_DIR = "../../noisy/"
TESTS_DIR = "../../tests/noise/"
noises = [None, "white", "brown", "pink", "green"]
# noises = [None]
# samples = ["love_kendrick_lamar"]  # Replace with actual sample names

def extract_top_samples(sample, noise, compressor):
    sample_path = Path(NOISY_DIR) / f"{sample}_{noise}.wav"
    if noise is None:
        sample_path = Path(NOISY_DIR) / f"{sample}.wav"
    cmd = [EXECUTABLE, "-s", str(sample_path), "-d", MUSIC_DIR, "-c", compressor]
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        output = result.stdout
        lines = output.splitlines()
        top_dominant = None
        for i, line in enumerate(lines):
            if "Top 4 closest music files (dominant frequencies):" in line and i + 1 < len(lines):
                match = re.search(r"\d+\.\s+([^\s]+)", lines[i+1])
                if match:
                    top_dominant = match.group(1)
        return top_dominant
    except subprocess.CalledProcessError as e:
        print(f"Erro ao processar {sample_path}: {e}")
    return None, None

def main():

    noisy_files = sorted(Path(NOISY_DIR).glob("*.wav"))
    samples = [f.name.replace(".wav", "") for f in noisy_files if not any(noise in f.name for noise in ["white", "brown", "pink", "green"])]

    compressors = ["gz", "bz2", "xz", "zstd", "fcm"]
    for compressor in compressors:
        print(f"Processing with compressor: {compressor}")
        output_dir = Path(TESTS_DIR) / compressor
        output_dir.mkdir(parents=True, exist_ok=True)
        for noise in noises:
            print(f"Extrating NCD values for {noise} noise")
            csv_path = output_dir / f"ncd_{noise}.csv"
            with open(csv_path, "w", newline="") as csvfile:
                writer = csv.writer(csvfile)
                writer.writerow(["SampleWithNoise", "MostSimilarSample"])
                for sample in samples:
                    top_dom = extract_top_samples(sample, noise, compressor)
                    if top_dom is not None:
                        top_dom = top_dom.replace(".wav", "")
                        writer.writerow([sample, top_dom])
                        print(f"\t[{noise}][{compressor}] {sample} -> Most Similar: {top_dom}")
                    else:
                        print(f"\t[{noise}][{compressor}] {sample} -> erro")

if __name__ == "__main__":
    main()
