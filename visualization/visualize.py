import argparse

import matplotlib.pyplot as plt
import numpy as np
from matplotlib import animation
from matplotlib.lines import Line2D
from matplotlib.text import Text

from utils import load_state_outputs


def main(folder: str, function: str) -> None:
    """Visualize the wave simulation."""
    # Load state data using Pydantic schemas
    states = load_state_outputs(folder)
    # Extract times and values
    times = [state.time for state in states]
    values = np.array([getattr(state, function) for state in states])

    # Create x-axis (assuming uniform grid from 0 to 1)
    n_points = len(values[0])
    x = np.linspace(0, 1, n_points)

    # Set up the figure and axis
    fig, ax = plt.subplots(figsize=(10, 6))
    ax.grid(visible=True, alpha=0.3)
    ax.set_xlim(0, 1)
    ax.set_ylim(np.min(values[0]) * 1.1, np.max(values[0]) * 1.1)
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

    def animate(frame: int) -> tuple[Line2D, Text]:
        """Create a frames with freezing at start and end."""
        prev_y_min, prev_y_max = ax.get_ylim()

        y_min = min(prev_y_min, np.min(values[frame]) * 1.1)
        y_max = max(prev_y_max, np.max(values[frame]) * 1.1)

        ax.set_ylim(y_min, y_max)
        line.set_data(x, values[frame])
        time_text.set_text(f"Time: {times[frame]:.4f}")
        return line, time_text

    # Create frames with freezing at start and end
    # Add 30 frames (1 second at 30fps) at the beginning and end
    fps = 30
    freeze_frames = fps
    total_frames = len(states) + 2 * freeze_frames

    def animate_with_freeze(frame: int) -> tuple[Line2D, Text]:
        """Create a frames with freezing at start and end."""
        if frame < freeze_frames:
            # Freeze on first frame
            return animate(0)
        if frame < freeze_frames + len(states):
            # Normal animation
            return animate(frame - freeze_frames)
        # Freeze on last frame
        return animate(len(states) - 1)

    # Create animation
    anim = animation.FuncAnimation(
        fig, animate_with_freeze, frames=total_frames, interval=50, blit=True, repeat=True
    )

    # Save as MP4
    anim.save(
        f"results/{folder}/{function}.mp4",
        writer="ffmpeg",
        fps=fps,
        bitrate=1000,
        extra_args=["-vcodec", "libx264"],
    )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Visualize the wave simulation.")
    parser.add_argument("folder", type=str, help="The folder to visualize.")
    parser.add_argument("function", type=str, help="The function to visualize.")
    args = parser.parse_args()
    main(args.folder, args.function)
