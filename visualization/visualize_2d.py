import argparse

import matplotlib.pyplot as plt
import numpy as np
from matplotlib import animation
from matplotlib.image import AxesImage
from matplotlib.text import Text
from mpl_toolkits.axes_grid1 import make_axes_locatable
from scipy.interpolate import interp1d

from utils import load_state_outputs


def main(folder: str, function: str, gamma: float) -> None:
    """Visualize the wave simulation in 2D."""
    states = load_state_outputs(folder)
    times = [state.time for state in states]
    values = np.array([getattr(state, function) for state in states])

    # Create radial grid
    n_points = 2 * len(values[0])
    r = np.linspace(0, 1, len(values[0]))

    # Create 2D grid for visualization
    x_2d = np.linspace(-1, 1, n_points)
    y_2d = np.linspace(-1, 1, n_points)

    x_grid, y_grid = np.meshgrid(x_2d, y_2d)
    r_grid = np.sqrt(x_grid**2 + y_grid**2)

    inside_domain = r_grid <= 1.0

    fig, ax = plt.subplots(figsize=(10, 10))
    ax.set_xlim(-1, 1)
    ax.set_ylim(-1, 1)
    ax.set_aspect("equal")
    ax.set_xlabel("X")
    ax.set_ylabel("Y")
    ax.set_title("Confined Space Simulation")

    img = ax.imshow(
        np.zeros((n_points, n_points)),
        extent=(-1, 1, -1, 1),
        origin="lower",
        cmap="viridis",
        vmin=np.nanmin(values),
        vmax=np.nanmax(values),
    )

    divider = make_axes_locatable(ax)
    cax = divider.append_axes("right", size="4%", pad=0.06)
    cbar = fig.colorbar(img, cax=cax)
    cbar.set_label(function.replace("_", " ").capitalize())

    time_text = ax.text(
        0.02,
        0.98,
        "",
        transform=ax.transAxes,
        fontsize=12,
        bbox={"boxstyle": "round", "facecolor": "wheat", "alpha": 0.8},
        verticalalignment="top",
    )

    plt.tight_layout()

    # Create interpolator that broadcasts across all frames at once
    # interp1d with axis parameter allows vectorized interpolation
    interpolator = interp1d(r, values, axis=1, bounds_error=False)

    # Flatten r to interpolate all points at once
    r_flat = r_grid.flatten()
    all_frames_flat = interpolator(r_flat)
    all_frames = all_frames_flat.reshape(len(states), n_points, n_points)

    all_frames = np.where(inside_domain, all_frames, np.nan)
    all_frames = _apply_gamma_correction(all_frames, gamma)

    def animate(frame: int) -> tuple[AxesImage, Text]:
        """Create a frames with freezing at start and end."""
        img.set_array(all_frames[frame])
        time_text.set_text(f"Time: {times[frame]:.4f}")
        return img, time_text

    def animate_with_freeze(frame: int) -> tuple[AxesImage, Text]:
        """Create a frames with freezing at start and end."""
        if frame < freeze_frames:
            return animate(0)
        if frame < freeze_frames + len(states):
            return animate(frame - freeze_frames)
        return animate(len(states) - 1)

    fps = 30
    freeze_frames = fps
    total_frames = len(states) + 2 * freeze_frames

    anim = animation.FuncAnimation(
        fig, animate_with_freeze, frames=total_frames, interval=50, blit=True, repeat=True
    )
    anim.save(
        f"{folder}/{function}_2d.mp4",
        writer="ffmpeg",
        fps=fps,
        bitrate=1000,
        extra_args=["-vcodec", "libx264"],
    )


def _apply_gamma_correction(values: np.ndarray, gamma: float) -> np.ndarray:
    if np.all(values >= 0):
        # Store original min/max for normalization
        original_min = np.nanmin(values)
        original_max = np.nanmax(values)

        # Apply gamma correction
        values = np.power(values, gamma)

        # Normalize back to original range to preserve colorbar scale
        gamma_min = np.nanmin(values)
        gamma_max = np.nanmax(values)
        values = original_min + (values - gamma_min) * (original_max - original_min) / (
            gamma_max - gamma_min
        )
    return values


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Visualize the wave simulation in 2D.")
    parser.add_argument("folder", type=str, help="The folder to visualize.")
    parser.add_argument("function", type=str, help="The function to visualize.")
    parser.add_argument(
        "--gamma",
        type=float,
        default=0.25,
        help="Gamma correction factor (default: 0.25). Only applied if all values are positive.",
    )
    args = parser.parse_args()
    main(args.folder, args.function, args.gamma)
