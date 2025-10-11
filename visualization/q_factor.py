import argparse

import matplotlib.pyplot as plt
import numpy as np

from utils import load_state_outputs


def main(low_folder: str, mid_folder: str, high_folder: str, function: str) -> None:
    """Visualize Q-factor for a given function at different levels of discretization."""
    _, ax = plt.subplots(figsize=(10, 6))
    ax.set_xlabel("Time", fontsize=14)
    ax.set_ylabel("Q-factor", fontsize=14)
    ax.set_title(f"Q-factor for {function.replace('_', ' ').capitalize()}", fontsize=16)

    high_data, times = _load_data(high_folder, function)
    mid_data, _ = _load_data(mid_folder, function)
    low_data, _ = _load_data(low_folder, function)

    max_len = min(len(high_data), len(mid_data), len(low_data))
    high_data = high_data[:max_len]
    mid_data = mid_data[:max_len]
    low_data = low_data[:max_len]
    times = times[:max_len]

    num = np.linalg.norm(mid_data - low_data, axis=1)
    denom = np.linalg.norm(high_data - mid_data, axis=1)
    with np.errstate(invalid="ignore"):  # Ignore division by zero. We want those to be NaN.
        q_factor = np.clip(num / denom, 0, 32)  # Clip noisy spikes near BH formation.

    ax.plot(times, q_factor, "b-", linewidth=2)
    ax.hlines(16, times[0], times[-1], "r", "--", linewidth=2)

    suffix = f"{high_folder}_{mid_folder}_{low_folder}"
    plt.savefig(f"../results/q_factor_{suffix}_{function}.png")


def _load_data(folder: str, function: str) -> tuple[np.ndarray, np.ndarray]:
    """Load data from a given folder and function."""
    states = load_state_outputs(folder)
    data = [getattr(state, function) for state in states]
    times = [state.time for state in states]
    return np.array(data), np.array(times)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Visualize conservation of energy at different levels of discretization."
    )
    parser.add_argument(
        "folders",
        type=str,
        help="The three folders to visualize, in order of lowest to highest resolution.",
        nargs=3,
    )
    parser.add_argument("function", type=str, help="The function to visualize.")
    args = parser.parse_args()
    main(args.folders[0], args.folders[1], args.folders[2], args.function)
