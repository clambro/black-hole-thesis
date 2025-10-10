import json
import matplotlib.pyplot as plt
import numpy as np
import argparse

def main(low_folder, mid_folder, high_folder, function):
    # Set up the figure and axis
    fig, ax = plt.subplots(figsize=(10, 6))
    ax.set_xlabel('Time', fontsize=14)
    ax.set_ylabel('Q-factor', fontsize=14)
    ax.set_title(f'Q-factor for {function.replace("_", " ").capitalize()}', fontsize=16)

    high_data, times = _load_data(high_folder, function)
    mid_data, _ = _load_data(mid_folder, function)
    low_data, _ = _load_data(low_folder, function)

    max_len = min(len(high_data), len(mid_data), len(low_data))
    high_data = high_data[:max_len]
    mid_data = mid_data[:max_len]
    low_data = low_data[:max_len]
    times = times[:max_len]

    q_factor = np.linalg.norm(mid_data - low_data, axis=1) / np.linalg.norm(high_data - mid_data, axis=1)
    q_factor = np.clip(q_factor, 0, 32)

    ax.plot(times, q_factor, 'b-', linewidth=2)
    ax.hlines(16, times[0], times[-1], 'r', '--', linewidth=2)

    suffix = "_".join([high_folder, mid_folder, low_folder])
    plt.savefig(f'results/q_factor_{suffix}_{function}.png')


def _load_data(folder, function):
    data = []
    times = []
    with open(f'results/{folder}/states.jsonl', 'r') as f:
        for line in f:
            json_data = json.loads(line.strip())
            data.append(json_data[function])
            times.append(json_data['time'])
    return np.array(data), np.array(times)



if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Visualize conservation of energy at different levels of discretization.')
    parser.add_argument('folders', type=str, help='The three folders to visualize, in order of lowest to highest resolution.', nargs=3)
    parser.add_argument('function', type=str, help='The function to visualize.')
    args = parser.parse_args()
    main(*args.folders, args.function)
