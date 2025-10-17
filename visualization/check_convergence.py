import argparse
from enum import StrEnum

import matplotlib.pyplot as plt
import numpy as np

from schemas import StateOutput
from utils import load_state_outputs


class CheckType(StrEnum):
    """Type of convergence check."""

    ENERGY = "energy"
    MASS_RESIDUAL = "mass_residual"


def main(*folders: str, check_type: CheckType) -> None:
    """Visualize convergence checks at different levels of discretization."""
    _, ax = plt.subplots(figsize=(10, 6))
    ax.set_xlabel("Time", fontsize=14)
    ax.set_ylabel(_get_ylabel(check_type), fontsize=14)
    ax.set_title(_get_title(check_type), fontsize=16)

    for folder in folders:
        states = load_state_outputs(folder)
        times = [state.time for state in states]

        if check_type == CheckType.ENERGY:
            values = _calculate_energy_deviation(states)
        else:
            values = _calculate_mass_residual(states)

        with np.errstate(divide="ignore"):  # Ignore log(0). We want those to be NaN.
            values = np.log(np.abs(values)) / np.log(16)

        ax.plot(times, values, linewidth=2, label=f"level={folder.split('_')[1]}")
        ax.legend()
        ax.grid(visible=True, which="major", linestyle="--", color="gray", alpha=0.5)

    suffix = "_".join(folders)
    filename = f"../results/{check_type}_{suffix}.png"
    plt.savefig(filename)


def _calculate_energy_deviation(states: list[StateOutput]) -> np.ndarray:
    """Calculate energy deviation from initial value."""
    initial_energy = states[0].total_energy
    return np.array([state.total_energy - initial_energy for state in states])


def _calculate_mass_residual(states: list[StateOutput]) -> np.ndarray:
    """Calculate mass equation residual (difference between mass calculations)."""
    mass_values = np.array([state.mass for state in states])
    alternate_mass_values = np.array([state.alternate_mass for state in states])
    return np.linalg.norm(mass_values - alternate_mass_values, axis=1)


def _get_ylabel(check_type: CheckType) -> str:
    """Get y-axis label based on check type."""
    if check_type == CheckType.ENERGY:
        return r"$\log_{16}(\Delta E_{total})$"
    return r"$\log_{16}(|P_{flux}|)$"


def _get_title(check_type: CheckType) -> str:
    """Get plot title based on check type."""
    if check_type == CheckType.ENERGY:
        return "Conservation of Energy"
    return "Mass Equation Residuals"


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Visualize convergence checks at different levels of discretization."
    )
    parser.add_argument("folders", type=str, help="The folders to visualize.", nargs="+")
    parser.add_argument("--type", choices=CheckType, help="Type of convergence check to perform.")
    args = parser.parse_args()
    main(*args.folders, check_type=args.type)
