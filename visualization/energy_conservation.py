import argparse

import matplotlib.pyplot as plt
import numpy as np

from utils import load_state_outputs


def main(*folders: str) -> None:
    """Visualize conservation of energy at different levels of discretization."""
    # Set up the figure and axis
    fig, ax = plt.subplots(figsize=(10, 6))
    ax.set_xlabel("Time", fontsize=14)
    ax.set_ylabel(r"$\log_{16}(\Delta E_{total})$", fontsize=14)
    ax.set_title("Conservation of Energy", fontsize=16)

    for folder in folders:
        states = load_state_outputs(folder)
        initial_energy = states[0].total_energy

        times = [state.time for state in states]
        values = [state.total_energy - initial_energy for state in states]
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
