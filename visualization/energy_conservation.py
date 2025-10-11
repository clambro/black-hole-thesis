import json
import matplotlib.pyplot as plt
import numpy as np
import argparse

def main(*folders: str):
    # Set up the figure and axis
    fig, ax = plt.subplots(figsize=(10, 6))
    ax.set_xlabel('Time', fontsize=14)
    ax.set_ylabel('$\log_{16}(\Delta E_{total})$', fontsize=14)
    ax.set_title('Conservation of Energy', fontsize=16)

    for folder in folders:
        data = []
        with open(f'results/{folder}/states.jsonl', 'r') as f:
            for line in f:
                data.append(json.loads(line.strip()))

        initial_energy = data[0]['total_energy']

        times = [d['time'] for d in data]
        values = [d['total_energy'] - initial_energy for d in data]
        values = np.log(np.abs(values)) / np.log(16)

        ax.plot(times, values, linewidth=2, label=f"level={folder.split('_')[1]}")
        ax.legend()

        ax.grid(True, which='major', linestyle='--', color='gray', alpha=0.5)

    suffix = "_".join(folders)
    plt.savefig(f'results/energy_conservation_{suffix}.png')



if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Visualize conservation of energy at different levels of discretization.')
    parser.add_argument('folders', type=str, help='The folders to visualize.', nargs='+')
    args = parser.parse_args()
    main(*args.folders)
