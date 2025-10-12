# Black Hole Simulation Visualization

## Scripts

### `visualize.py` - 1D Wave Animation

Creates animated line plots showing wave evolution over time.

**Usage:**

```bash
python visualize.py <folder> <function>
```

**Parameters:**

- `folder`: Results folder name (e.g., "38_10")
- `function`: Field to visualize (e.g., "field", "mass", "energy_density")

### `visualize_2d.py` - 2D Wave Animation

Creates animated 2D heatmap visualizations with gamma correction.

**Usage:**

```bash
python visualize_2d.py <folder> <function> [--gamma GAMMA]
```

**Parameters:**

- `folder`: Results folder name (e.g., "38_10")
- `function`: Field to visualize (e.g., "field", "mass", "energy_density")
- `--gamma`: Gamma correction factor (default: 0.25)

### `check_convergence.py` - Convergence Analysis

Analyzes energy conservation and mass equation residuals across different resolution levels.

**Usage:**

```bash
python check_convergence.py <folders...> [--type TYPE]
```

**Parameters:**

- `folders`: Multiple result folders to compare (e.g., "38_8 38_9 38_10")
- `--type`: Analysis type - `energy` or `mass_residual` (default: energy)

### `q_factor.py` - Q-Factor Analysis

Calculates and visualizes Q-factors for convergence analysis.

**Usage:**

```bash
python q_factor.py <low_folder> <mid_folder> <high_folder> <function>
```

**Parameters:**

- `low_folder`: Lowest resolution folder (e.g., "38_8")
- `mid_folder`: Medium resolution folder (e.g., "38_9")
- `high_folder`: Highest resolution folder (e.g., "38_10")
- `function`: Field to analyze (e.g., "field", "mass", "energy_density")

### `analyze_results.py` - Critical Scaling Analysis

Analyzes black hole formation across multiple initial amplitudes, identifying families of solutions and fitting power-law scaling relationships.

**Usage:**

```bash
python analyze_results.py
```

**Features:**

- **Family Grouping**: Automatically groups simulation results into families based on formation times (within 1 time unit)
- **Power Law Fitting**: Fits curves of the form `M = C × (ε - ε*)^γ` to each family
- **Uncertainty Quantification**: Uses 10,000 bootstrap iterations with proper accounting for:
  - Variability across different critical amplitudes (ε\*)
  - Within-fit uncertainty from regression residuals
  - Covariance between γ and intercept coefficients
- **95% Confidence Intervals**: Displays uncertainty bands on fitted curves
- **Monotonic Filtering**: Only fits the monotonically increasing portion of each family

**Output:**

- `results/result.png`: Dual plot showing BH mass and formation time vs. initial amplitude with fitted curves
- `results/fit_results.jsonl`: JSON Lines file containing fit parameters (γ, ε\*, confidence intervals)

## Data Structure

The scripts expect simulation results in the following structure:

```
results/
├── <folder>/
│   ├── results.json  # Final results
│   └── states.jsonl  # Time series data
```

## Output

- **Animations**: MP4 files saved to `results/<folder>/`
- **Plots**: PNG files saved to `results/`
- **Video Quality**: 1500x900 pixels at 30 FPS with 2 Mbps bitrate
