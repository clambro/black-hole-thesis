#!/usr/bin/env python3
import json
import matplotlib.pyplot as plt
import matplotlib.animation as animation
import numpy as np

# Read the JSONL file
data = []
with open('results/simulation_output.jsonl', 'r') as f:
    for line in f:
        data.append(json.loads(line.strip()))

# Extract times and positions
times = [d['time'] for d in data]
positions = [d['position'] for d in data]

# Create x-axis (assuming uniform grid from 0 to 1)
n_points = len(positions[0])
x = np.linspace(0, 1, n_points)

# Set up the figure and axis
fig, ax = plt.subplots(figsize=(10, 6))
ax.set_xlim(0, 1)
ax.set_ylim(min(min(pos) for pos in positions) * 1.1, 
            max(max(pos) for pos in positions) * 1.1)
ax.set_xlabel('Position (x)')
ax.set_ylabel('Wave Amplitude')
ax.set_title('Wave Simulation')
ax.grid(True, alpha=0.3)

line, = ax.plot([], [], 'b-', linewidth=2)
time_height = 0.9 * ax.get_ylim()[1]
time_text = ax.text(0.02, time_height, '', transform=ax.transAxes, fontsize=12,
                   bbox=dict(boxstyle='round', facecolor='wheat', alpha=0.8))

def animate(frame):
    line.set_data(x, positions[frame])
    time_text.set_text(f'Time: {times[frame]:.4f}')
    return line, time_text

# Create animation
anim = animation.FuncAnimation(fig, animate, frames=len(data), 
                              interval=50, blit=True, repeat=True)

# Save as MP4
print("Saving animation...")
anim.save('results/wave_simulation.mp4', writer='ffmpeg', fps=30, bitrate=1000, extra_args=['-vcodec', 'libx264'])
print("Animation saved as wave_simulation.mp4")
