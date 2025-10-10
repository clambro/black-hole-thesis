import json
import matplotlib.pyplot as plt
import matplotlib.animation as animation
import numpy as np
import argparse

def main(folder: str, function: str):
    # Read the JSONL file
    data = []
    with open(f'results/{folder}/states.jsonl', 'r') as f:
        for line in f:
            data.append(json.loads(line.strip()))

    # Extract times and values
    times = [d['time'] for d in data]
    values = np.array([d[function] for d in data])

    # Create x-axis (assuming uniform grid from 0 to 1)
    n_points = len(values[0])
    x = np.linspace(0, 1, n_points)

    # Set up the figure and axis
    fig, ax = plt.subplots(figsize=(10, 6))
    ax.grid(True, alpha=0.3)
    ax.set_xlim(0, 1)
    ax.set_ylim(np.min(values[0]) * 1.1, np.max(values[0]) * 1.1)
    ax.set_xlabel('Position')
    ax.set_ylabel(function.replace('_', ' ').capitalize())
    ax.set_title('Confined Space Simulation')

    line, = ax.plot([], [], 'b-', linewidth=2)
    time_text = ax.text(0.8, 0.8, '', transform=ax.transAxes, fontsize=12,
                    bbox=dict(boxstyle='round', facecolor='wheat', alpha=0.8))

    def animate(frame):
        prev_y_min, prev_y_max = ax.get_ylim()

        y_min = min(prev_y_min, np.min(values[frame]) * 1.1)
        y_max = max(prev_y_max, np.max(values[frame]) * 1.1)

        ax.set_ylim(y_min, y_max)
        line.set_data(x, values[frame])
        time_text.set_text(f'Time: {times[frame]:.4f}')
        return line, time_text

    # Create frames with freezing at start and end
    # Add 30 frames (1 second at 30fps) at the beginning and end
    fps = 30
    freeze_frames = fps
    total_frames = len(data) + 2 * freeze_frames

    def animate_with_freeze(frame):
        if frame < freeze_frames:
            # Freeze on first frame
             return animate(0)
        elif frame < freeze_frames + len(data):
            # Normal animation
             return animate(frame - freeze_frames)
        else:
            # Freeze on last frame
             return animate(len(data) - 1)

    # Create animation
    anim = animation.FuncAnimation(fig, animate_with_freeze, frames=total_frames,
                                interval=50, blit=True, repeat=True)

    # Save as MP4
    print("Saving animation...")
    anim.save(f'results/{folder}/{function}.mp4', writer='ffmpeg', fps=fps, bitrate=1000, extra_args=['-vcodec', 'libx264'])
    print(f"Animation saved as {folder}/{function}.mp4")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Visualize the wave simulation.')
    parser.add_argument('folder', type=str, help='The folder to visualize.')
    parser.add_argument('function', type=str, help='The function to visualize.')
    args = parser.parse_args()
    main(args.folder, args.function)
