import argparse

import matplotlib.pyplot as plt
import numpy as np
from matplotlib import animation
from matplotlib.figure import Figure
from matplotlib.image import AxesImage
from matplotlib.text import Text
from mpl_toolkits.axes_grid1 import make_axes_locatable
from scipy.interpolate import interp1d
from tqdm import tqdm

from schemas import StateOutput
from utils import load_state_outputs

FPS = 30


def main(folder: str, function: str, gamma: float) -> None:
    """Visualize the wave simulation in 2D."""
    states = load_state_outputs(folder)
    times, values = _extract_data(states, function)

    anim = _create_2d_animation(function, times, values, gamma, states)
    _save_2d_animation(anim, folder, function)


def _extract_data(states: list[StateOutput], function: str) -> tuple[list[float], np.ndarray]:
    """Extract time and field data from states."""
    times = [state.time for state in states]
    values = np.array([getattr(state, function) for state in states])
    return times, values


def _create_2d_animation(
    function: str,
    times: list[float],
    values: np.ndarray,
    gamma: float,
    states: list[StateOutput],
) -> animation.FuncAnimation:
    """Create the 2D animation with freeze frames."""
    r, r_grid = _create_2d_grid(values)

    fig, img, time_text = _setup_2d_plot(function, values)
    all_frames = _prepare_2d_data(values, r, r_grid, gamma)

    freeze_seconds = 1
    freeze_frames = freeze_seconds * FPS
    total_frames = len(states) + 2 * freeze_seconds * FPS

    pbar = tqdm(total=total_frames, desc="Generating 2D animation", unit="frames")

    def animate(frame: int) -> tuple[AxesImage, Text]:
        """Animate a single frame."""
        img.set_array(all_frames[frame])
        time_text.set_text(f"Time: {times[frame]:.4f}")
        pbar.update(1)
        return img, time_text

    def animate_with_freeze(frame: int) -> tuple[AxesImage, Text]:
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


def _create_2d_grid(
    values: np.ndarray,
) -> tuple[np.ndarray, np.ndarray]:
    """Create 2D grid for visualization."""
    r = np.linspace(0, 1, len(values[0]))

    x_2d = np.linspace(-1, 1, len(values[0]))
    y_2d = np.linspace(-1, 1, len(values[0]))
    x_grid, y_grid = np.meshgrid(x_2d, y_2d)
    r_grid = np.sqrt(x_grid**2 + y_grid**2)

    return r, r_grid


def _setup_2d_plot(function: str, values: np.ndarray) -> tuple[Figure, AxesImage, Text]:
    """Set up the 2D plot with axes, colorbar, and initial elements."""
    fig, ax = plt.subplots(figsize=(7, 6), dpi=150)
    ax.set_xlim(-1, 1)
    ax.set_ylim(-1, 1)
    ax.set_aspect("equal")
    ax.set_xlabel("X")
    ax.set_ylabel("Y")
    ax.set_title("Confined Space Simulation")

    img = ax.imshow(
        np.zeros((len(values[0]), len(values[0]))),
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
    return fig, img, time_text


def _prepare_2d_data(
    values: np.ndarray,
    r: np.ndarray,
    r_grid: np.ndarray,
    gamma: float,
) -> np.ndarray:
    """Prepare 2D data for animation by interpolating and applying corrections."""
    interpolator = interp1d(r, values, axis=1, bounds_error=False)
    r_flat = r_grid.flatten()
    all_frames_flat = interpolator(r_flat)
    all_frames = all_frames_flat.reshape(values.shape[0], r_grid.shape[0], r_grid.shape[1])

    all_frames = np.where(r_grid <= 1.0, all_frames, np.nan)
    return _apply_gamma_correction(all_frames, gamma)


def _apply_gamma_correction(values: np.ndarray, gamma: float) -> np.ndarray:
    if np.all(values[~np.isnan(values)] >= 0):
        original_min = np.nanmin(values)
        original_max = np.nanmax(values)

        values = np.power(values, gamma)

        gamma_min = np.nanmin(values)
        gamma_max = np.nanmax(values)
        values = original_min + (values - gamma_min) * (original_max - original_min) / (
            gamma_max - gamma_min
        )
    return values


def _save_2d_animation(anim: animation.FuncAnimation, folder: str, function: str) -> None:
    """Save the 2D animation to file."""
    anim.save(
        f"../results/{folder}/{function}_2d.mp4",
        writer="ffmpeg",
        fps=FPS,
        bitrate=2000,
        extra_args=["-vcodec", "libx264"],
    )


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
