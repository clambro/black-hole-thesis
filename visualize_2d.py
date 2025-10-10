import json
import matplotlib.pyplot as plt
import matplotlib.animation as animation
import numpy as np
import argparse
from mpl_toolkits.axes_grid1 import make_axes_locatable

def main(folder: str, function: str, gamma: float = 0.5):
    # Read the JSONL file
    data = []
    with open(f'results/{folder}/states.jsonl', 'r') as f:
        for line in f:
            data.append(json.loads(line.strip()))

    # Extract times and values
    times = [d['time'] for d in data]
    values = np.array([d[function] for d in data])
    
    # Create radial grid (assuming uniform grid from 0 to 1)
    n_points = 2 * len(values[0])
    r = np.linspace(0, 1, len(values[0]))  # Keep original radial grid for interpolation
    
    # Create 2D grid for visualization
    # We'll use a square grid and map radial coordinates to it
    x_2d = np.linspace(-1, 1, n_points)
    y_2d = np.linspace(-1, 1, n_points)
    X, Y = np.meshgrid(x_2d, y_2d)
    
    # Calculate radial distance from center for each point
    R = np.sqrt(X**2 + Y**2)
    
    # Create mask for points inside the circular domain (r <= 1)
    inside_domain = R <= 1.0
    
    # Set up the figure and axis
    fig, ax = plt.subplots(figsize=(10, 10))
    ax.set_xlim(-1, 1)
    ax.set_ylim(-1, 1)
    ax.set_aspect('equal')
    ax.set_xlabel('X')
    ax.set_ylabel('Y')
    ax.set_title('Confined Space Simulation')
    
    # Check if all values are positive for gamma correction
    all_positive = np.all(values >= 0)
    
    # Initialize the image
    img = ax.imshow(np.zeros((n_points, n_points)), 
                    extent=[-1, 1, -1, 1], 
                    origin='lower', 
                    cmap='viridis', 
                    vmin=np.nanmin(values), 
                    vmax=np.nanmax(values))
    
    # Colorbar with the same height as the axes
    divider = make_axes_locatable(ax)
    cax = divider.append_axes("right", size="4%", pad=0.06)
    cbar = fig.colorbar(img, cax=cax)
    cbar.set_label(function.replace('_', ' ').capitalize())
    
    # Add time text
    time_text = ax.text(0.02, 0.98, '', transform=ax.transAxes, fontsize=12,
                        bbox=dict(boxstyle='round', facecolor='wheat', alpha=0.8),
                        verticalalignment='top')

    plt.tight_layout()
    
    # Precompute all frames using fully vectorized operations
    print("Precomputing all frames...")
    
    # Fully vectorized interpolation using scipy's interp1d
    from scipy.interpolate import interp1d
    
    # Create interpolator that broadcasts across all frames at once
    # interp1d with axis parameter allows vectorized interpolation
    interpolator = interp1d(r, values, axis=1, bounds_error=False, fill_value='extrapolate')
    
    # Flatten R to interpolate all points at once
    R_flat = R.flatten()
    
    # Interpolate all frames and all points in one vectorized operation
    all_frames_flat = interpolator(R_flat)  # Shape: (n_frames, n_points^2)
    
    # Reshape to (n_frames, n_points, n_points)
    all_frames = all_frames_flat.reshape(len(data), n_points, n_points)
    
    # Apply domain mask: set points outside domain to NaN
    all_frames = np.where(inside_domain, all_frames, np.nan)
    
    # Apply gamma correction if all values are positive
    if all_positive:
        # Store original min/max for normalization
        original_min = np.nanmin(values)
        original_max = np.nanmax(values)
        
        # Apply gamma correction
        all_frames = np.power(all_frames, gamma)
        
        # Normalize back to original range to preserve colorbar scale
        gamma_min = np.nanmin(all_frames)
        gamma_max = np.nanmax(all_frames)
        all_frames = original_min + (all_frames - gamma_min) * (original_max - original_min) / (gamma_max - gamma_min)
    
    def animate(frame):
        img.set_array(all_frames[frame])
        time_text.set_text(f'Time: {times[frame]:.4f}')
        return img, time_text

    # Create frames with freezing at start and end
    # Add 30 frames (1 second at 30fps) at the beginning and end
    fps = 30
    freeze_frames = fps
    total_frames = len(data) + 2 * freeze_frames    
    
    def animate_with_freeze(frame):
        if frame < freeze_frames:
            return animate(0)
        elif frame < freeze_frames + len(data):
            return animate(frame - freeze_frames)
        else:
            return animate(len(data) - 1)
    
    # Create animation
    anim = animation.FuncAnimation(fig, animate_with_freeze, frames=total_frames, 
                                interval=50, blit=True, repeat=True)

    # Save as MP4
    print("Saving 2D animation...")
    anim.save(f'results/{folder}/{function}_2d.mp4', writer='ffmpeg', fps=fps, bitrate=1000, extra_args=['-vcodec', 'libx264'])
    print(f"2D animation saved as {folder}/{function}_2d.mp4")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Visualize the wave simulation in 2D.')
    parser.add_argument('folder', type=str, help='The folder to visualize.')
    parser.add_argument('function', type=str, help='The function to visualize.')
    parser.add_argument('--gamma', type=float, default=0.25, 
                       help='Gamma correction factor (default: 0.25). Only applied if all values are positive.')
    args = parser.parse_args()
    main(args.folder, args.function, args.gamma)
