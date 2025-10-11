import argparse
import json
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def main(*folders: str) -> None:
    """Visualize conservation of energy at different levels of discretization."""
    # Set up the figure and axis
    fig, ax = plt.subplots(figsize=(10, 6))
    ax.set_xlabel("Time", fontsize=14)
    ax.set_ylabel(r"$\log_{16}(\Delta E_{total})$", fontsize=14)
    ax.set_title("Conservation of Energy", fontsize=16)

    for folder in folders:
        data = []
        with Path(f"results/{folder}/states.jsonl").open() as f:
            data.extend(json.loads(line.strip()) for line in f)

        initial_energy = data[0]["total_energy"]

        times = [d["time"] for d in data]
        values = [d["total_energy"] - initial_energy for d in data]
        values = np.log(np.abs(values)) / np.log(16)

        ax.plot(times, values, linewidth=2, label=f"level={folder.split('_')[1]}")
        ax.legend()

        ax.grid(visible=True, which="major", linestyle="--", color="gray", alpha=0.5)

    suffix = "_".join(folders)
    plt.savefig(f"results/energy_conservation_{suffix}.png")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Visualize conservation of energy at different levels of discretization."
    )
    parser.add_argument("folders", type=str, help="The folders to visualize.", nargs="+")
    args = parser.parse_args()
    main(*args.folders)
