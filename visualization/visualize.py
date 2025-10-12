import argparse

import matplotlib.pyplot as plt
import numpy as np
from matplotlib import animation
from matplotlib.axes import Axes
from matplotlib.figure import Figure
from matplotlib.lines import Line2D
from matplotlib.text import Text
from tqdm import tqdm

from schemas import StateOutput
from utils import load_state_outputs

FPS = 30


def main(folder: str, function: str) -> None:
    """Visualize the wave simulation."""
    states = load_state_outputs(folder)
    times, values, radius = _extract_data(states, function)

    anim = _create_animation(function, times, values, radius, states)
    _save_animation(anim, folder, function)


def _extract_data(
    states: list[StateOutput],
    function: str,
) -> tuple[list[float], np.ndarray, np.ndarray]:
    """Extract time and field data from states."""
    times = [state.time for state in states]
    values = np.array([getattr(state, function) for state in states])
    radius = np.linspace(0, 1, len(values[0]))
    return times, values, radius


def _create_animation(
    function: str,
    times: list[float],
    values: np.ndarray,
    radius: np.ndarray,
    states: list[StateOutput],
) -> animation.FuncAnimation:
    """Create the animation with freeze frames."""
    fig, ax, line, time_text = _setup_plot(function, values)
    freeze_seconds = 1
    freeze_frames = freeze_seconds * FPS
    total_frames = len(states) + 2 * freeze_seconds * FPS

    pbar = tqdm(total=total_frames, desc="Generating animation", unit="frames")

    def animate(frame: int) -> tuple[Line2D, Text]:
        """Animate a single frame."""
        prev_y_min, prev_y_max = ax.get_ylim()
        new_y_min, new_y_max = _get_extrema(values[frame])
        y_min = min(prev_y_min, new_y_min)
        y_max = max(prev_y_max, new_y_max)

        ax.set_ylim(y_min, y_max)
        line.set_data(radius, values[frame])
        time_text.set_text(f"Time: {times[frame]:.4f}")
        pbar.update(1)
        return line, time_text

    def animate_with_freeze(frame: int) -> tuple[Line2D, Text]:
        """Create frames with freezing at start and end."""
        if frame < freeze_frames:
            result = animate(0)
        elif frame < freeze_frames + len(states):
            result = animate(frame - freeze_frames)
        else:
            result = animate(len(states) - 1)
        return result

    return animation.FuncAnimation(
        fig, animate_with_freeze, frames=total_frames, interval=50, blit=True, repeat=True
    )


def _setup_plot(function: str, values: np.ndarray) -> tuple[Figure, Axes, Line2D, Text]:
    """Set up the plot with axes, labels, and initial elements."""
    fig, ax = plt.subplots(figsize=(10, 6), dpi=150)
    ax.grid(visible=True, alpha=0.3)
    ax.set_xlim(0, 1)
    ax.set_ylim(_get_extrema(values[0]))
    ax.set_xlabel("Position")
    ax.set_ylabel(function.replace("_", " ").capitalize())
    ax.set_title("Confined Space Simulation")

    (line,) = ax.plot([], [], "b-", linewidth=2)
    time_text = ax.text(
        0.8,
        0.8,
        "",
        transform=ax.transAxes,
        fontsize=12,
        bbox={"boxstyle": "round", "facecolor": "wheat", "alpha": 0.8},
    )

    return fig, ax, line, time_text


def _get_extrema(values: np.ndarray) -> tuple[float, float]:
    """Get the extremum of the values with some buffer for the axes limits."""
    min_value = np.min(values)
    max_value = np.max(values)
    min_value = np.sign(min_value) * np.abs(min_value) * 0.9
    max_value = np.sign(max_value) * np.abs(max_value) * 1.1
    return min_value, max_value


def _save_animation(anim: animation.FuncAnimation, folder: str, function: str) -> None:
    """Save the animation to file."""
    anim.save(
        f"../results/{folder}/{function}.mp4",
        writer="ffmpeg",
        fps=FPS,
        bitrate=2000,
        extra_args=["-vcodec", "libx264"],
    )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Visualize the wave simulation.")
    parser.add_argument("folder", type=str, help="The folder to visualize.")
    parser.add_argument("function", type=str, help="The function to visualize.")
    args = parser.parse_args()
    main(args.folder, args.function)
