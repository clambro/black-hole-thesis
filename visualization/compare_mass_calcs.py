import argparse

import matplotlib.pyplot as plt
import numpy as np

from utils import load_state_outputs


def main(*folders: str) -> None:
    """Visualize mass equation residuals at different levels of discretization."""
    # Set up the figure and axis
    fig, ax = plt.subplots(figsize=(10, 6))
    ax.set_xlabel("Time", fontsize=14)
    ax.set_ylabel(r"$\log_{16}(|P_{flux}|)$", fontsize=14)
    ax.set_title("Mass Equation Residuals", fontsize=16)

    for folder in folders:
        states = load_state_outputs(folder)
        times = [state.time for state in states]
        mass_values = np.array([state.mass for state in states])
        alternate_mass_values = np.array([state.alternate_mass for state in states])

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
