import re
from collections import defaultdict
from pathlib import Path
from typing import cast

import matplotlib.pyplot as plt
from pydantic import ValidationError

from schemas import SimulationOutput
from utils import load_simulation_output


def main() -> None:
    """Main function to orchestrate the plotting process."""
    script_dir = Path(__file__).parent
    project_root = script_dir.parent
    results_dir = project_root / "results"
    output_path = results_dir / "result.png"

    all_results = _load_results(results_dir)
    deduplicated = _deduplicate_results(all_results)
    _create_plots(deduplicated, output_path)


def _load_results(results_dir: Path) -> list[SimulationOutput]:
    """Load all results from the results directory."""
    pattern = re.compile(r"^\d+(?:\.\d+)?_\d+(?:\.\d+)?$")
    results: list[SimulationOutput] = []
    for item in results_dir.iterdir():
        if item.is_dir() and pattern.match(item.name):
            try:
                results.append(load_simulation_output(item.name))
            except (FileNotFoundError, ValidationError):
                continue
    return results


def _deduplicate_results(results: list[SimulationOutput]) -> list[SimulationOutput]:
    """Keep only unique initial amplitudes, preferring higher discretization."""
    # Group by initial amplitude
    grouped: dict[float, list[SimulationOutput]] = defaultdict(list)
    for result in results:
        grouped[result.initial_amplitude].append(result)

    # For each amplitude, keep the one with highest grid_level
    deduplicated: list[SimulationOutput] = []
    for group in grouped.values():
        best_result = max(group, key=lambda r: r.grid_level)
        deduplicated.append(best_result)

    return deduplicated


def _create_plots(results: list[SimulationOutput], output_path: Path) -> None:
    """Create the dual plot showing BH Mass and Formation Time vs epsilon."""
    amplitudes = [r.initial_amplitude for r in results]
    bh_masses = [r.black_hole_mass for r in results]
    formation_times = [r.final_simulation_time for r in results]

    if None in bh_masses:
        raise ValueError("None in bh_masses")
    bh_masses = cast("list[float]", bh_masses)

    # Create figure with two subplots
    _, (ax1, ax2) = plt.subplots(2, 1, figsize=(10, 8), sharex=True)

    # Top plot: BH Mass vs epsilon
    ax1.scatter(amplitudes, bh_masses, s=20, alpha=0.7, color="blue")
    ax1.set_ylabel("Black Hole Mass")
    ax1.set_ylim(0, max(bh_masses) * 1.1 if bh_masses else 0.02)
    ax1.grid(visible=True, alpha=0.3)

    # Bottom plot: Formation Time vs epsilon
    ax2.scatter(amplitudes, formation_times, s=20, alpha=0.7, color="blue")
    ax2.set_xlabel("Initial Amplitude")
    ax2.set_ylabel("Formation Time")
    ax2.set_ylim(0, max(formation_times) * 1.1 if formation_times else 15)
    ax2.grid(visible=True, alpha=0.3)

    # Set x-axis limits to match the image
    min_amp = min(amplitudes) if amplitudes else 18
    max_amp = max(amplitudes) if amplitudes else 38
    ax1.set_xlim(min_amp - 0.5, max_amp + 0.5)
    ax2.set_xlim(min_amp - 0.5, max_amp + 0.5)

    # Adjust layout and save
    plt.tight_layout()
    plt.savefig(output_path, dpi=300, bbox_inches="tight")


if __name__ == "__main__":
    main()
