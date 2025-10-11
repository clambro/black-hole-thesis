import argparse

import matplotlib.pyplot as plt
import numpy as np

from utils import load_state_outputs


def main(folder: str, time: float) -> None:
    """Visualize initial conditions from state output."""
    # Load state data and find the state at the specified time
    states = load_state_outputs(folder)
    initial_state = None
    for state in states:
        if state.time >= time:
            initial_state = state
            break
    if initial_state is None:
        raise ValueError(f"No state found at time {time} or later")

    # Define all the fields to visualize
    fields = [
        "field",
        "radial_gradient",
        "conj_momentum",
        "mass",
        "radial_factor",
        "lapse",
        "char_speed",
        "energy_density",
    ]

    # Create x-axis (uniform grid from 0 to 1)
    n_points = len(initial_state.radial_gradient)
    x = np.linspace(0, 1, n_points)

    # Create subplots - 3 rows, 3 columns
    fig, axes = plt.subplots(3, 3, figsize=(15, 12))
    fig.suptitle(f"Initial Conditions (t={initial_state.time:.4f})", fontsize=16)

    # Flatten axes for easier indexing
    axes = axes.flatten()

    # Plot each field
    for i, field in enumerate(fields):
        ax = axes[i]
        values = getattr(initial_state, field)

        ax.plot(x, values, "b-", linewidth=2)
        ax.set_xlabel("Radius")
        ax.set_ylabel(field.replace("_", " ").title())
        ax.set_title(field.replace("_", " ").title())
        ax.grid(visible=True, alpha=0.3)
        ax.set_xlim(0, 1)

    plt.tight_layout(rect=(0, 0, 1, 0.96))

    # Save the figure
    output_file = f"results/{folder}/initial_conditions.png"
    plt.savefig(output_file, dpi=300, bbox_inches="tight")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Visualize initial conditions from state output.")
    parser.add_argument("folder", type=str, help="The results folder to visualize (e.g., 38_10)")
    parser.add_argument(
        "time", type=float, default=0, help="The time to visualize the conditions at."
    )
    args = parser.parse_args()
    main(args.folder, args.time)
