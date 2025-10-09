import json
import matplotlib.pyplot as plt
import numpy as np
import argparse

def main(folder: str):
    # Read the first line of the JSONL file (initial condition)
    with open(f'results/{folder}/states.jsonl', 'r') as f:
        while True:
            json_data = json.loads(f.readline().strip())
            if json_data["time"] >= 0.5:
                break
        initial_state = json_data
    
    # Define all the fields to visualize
    fields = [ 
        'field',
        'radial_gradient',
        'conj_momentum',
        'mass',
        'compactness',
        'lapse',
        'char_speed',
        'energy_density'
    ]
    
    # Create x-axis (uniform grid from 0 to 1)
    n_points = len(initial_state['radial_gradient'])
    x = np.linspace(0, 1, n_points)
    
    # Create subplots - 3 rows, 3 columns
    fig, axes = plt.subplots(3, 3, figsize=(15, 12))
    fig.suptitle(f'Initial Conditions (t={initial_state["time"]:.4f})', fontsize=16)
    
    # Flatten axes for easier indexing
    axes = axes.flatten()
    
    # Plot each field
    for i, field in enumerate(fields):
        ax = axes[i]
        values = initial_state[field]
        
        ax.plot(x, values, 'b-', linewidth=2)
        ax.set_xlabel('Radius')
        ax.set_ylabel(field.replace('_', ' ').title())
        ax.set_title(field.replace('_', ' ').title())
        ax.grid(True, alpha=0.3)
        ax.set_xlim(0, 1)
    
    plt.tight_layout(rect=[0, 0, 1, 0.96])
    
    # Save the figure
    output_file = f'results/{folder}/initial_conditions.png'
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    print(f"Initial conditions plot saved to {output_file}")
    


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Visualize initial conditions from state output.')
    parser.add_argument('folder', type=str, help='The results folder to visualize (e.g., 38_10)')
    args = parser.parse_args()
    main(args.folder)