#!/usr/bin/env python3
import json
import matplotlib.pyplot as plt
import matplotlib.animation as animation
import numpy as np
import argparse

def main(function: str):
    # Read the JSONL file
    data = []
    with open('results/simulation_output.jsonl', 'r') as f:
        for line in f:
            data.append(json.loads(line.strip()))

    # Extract times and values
    times = [d['time'] for d in data]
    values = [d[function] for d in data]

    # Create x-axis (assuming uniform grid from 0 to 1)
    n_points = len(values[0])
    x = np.linspace(0, 1, n_points)

    # Set up the figure and axis
    fig, ax = plt.subplots(figsize=(10, 6))
    ax.set_xlim(0, 1)
    ax.set_ylim(min(min(pos) for pos in values) * 1.1, 
                max(max(pos) for pos in values) * 1.1)
    ax.set_xlabel('Position')
    ax.set_ylabel(function.replace('_', ' ').capitalize())
    ax.set_title('Wave Simulation')

    line, = ax.plot([], [], 'b-', linewidth=2)
    time_text = ax.text(0.8, 0.8, '', transform=ax.transAxes, fontsize=12,
                    bbox=dict(boxstyle='round', facecolor='wheat', alpha=0.8))

    def animate(frame):
        line.set_data(x, values[frame])
        time_text.set_text(f'Time: {times[frame]:.4f}')
        return line, time_text

    # Create animation
    anim = animation.FuncAnimation(fig, animate, frames=len(data), 
                                interval=50, blit=True, repeat=True)

    # Save as MP4
    print("Saving animation...")
    anim.save(f'results/{function}.mp4', writer='ffmpeg', fps=30, bitrate=1000, extra_args=['-vcodec', 'libx264'])
    print(f"Animation saved as {function}.mp4")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Visualize the wave simulation.')
    parser.add_argument('function', type=str, help='The function to visualize.')
    args = parser.parse_args()
    main(args.function)
