import argparse
import json
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def main(*folders: str) -> None:
    """Visualize mass equation residuals at different levels of discretization."""
    # Set up the figure and axis
    fig, ax = plt.subplots(figsize=(10, 6))
    ax.set_xlabel("Time", fontsize=14)
    ax.set_ylabel(r"$\log_{16}(|P_{flux}|)$", fontsize=14)
    ax.set_title("Mass Equation Residuals", fontsize=16)

    for folder in folders:
        data = []
        with Path(f"results/{folder}/states.jsonl").open() as f:
            data.extend(json.loads(line.strip()) for line in f)

        times = [d["time"] for d in data]
        mass_values = np.array([d["mass"] for d in data])
        alternate_mass_values = np.array([d["alternate_mass"] for d in data])

        diff = np.linalg.norm(mass_values - alternate_mass_values, axis=1)
        diff = np.log(np.abs(diff)) / np.log(16)
        ax.plot(times, diff, linewidth=2, label=f"level={folder.split('_')[1]}")

        ax.legend()
        ax.grid(visible=True, which="major", linestyle="--", color="gray", alpha=0.5)

    suffix = "_".join(folders)
    plt.savefig(f"results/mass_equation_residuals_{suffix}.png")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Visualize mass equation residuals at different levels of discretization."
    )
    parser.add_argument("folders", type=str, help="The folders to visualize.", nargs="+")
    args = parser.parse_args()
    main(*args.folders)
