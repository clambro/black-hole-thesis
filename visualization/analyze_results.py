import json
import re
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from matplotlib import cm
from pydantic import ValidationError

from schemas import SimulationOutput
from utils import load_simulation_output

RNG = np.random.default_rng(2112)


@dataclass
class DataPoint:
    """Single data point from simulation results."""

    amplitude: float
    bh_mass: float
    formation_time: float


@dataclass
class Family:
    """A family of data points grouped by formation time."""

    points: list[DataPoint]
    min_amplitude: float
    max_amplitude: float


@dataclass
class FitResult:
    """Results from fitting a power law to a family."""

    gamma: float
    gamma_min_95: float
    gamma_max_95: float
    amp_star: float
    amp_star_min: float
    amp_star_max: float
    fitted_amplitudes: np.ndarray
    fitted_masses: np.ndarray
    fitted_masses_lower: np.ndarray
    fitted_masses_upper: np.ndarray


def main() -> None:
    """Main function to orchestrate the plotting process."""
    script_dir = Path(__file__).parent
    project_root = script_dir.parent
    results_dir = project_root / "results"
    output_img_path = results_dir / "result.png"
    output_fit_results_path = results_dir / "fit_results.jsonl"

    all_results = _load_results(results_dir)
    data_points = _deduplicate_and_format_results(all_results)
    families = _group_into_families(data_points)
    fit_results = _fit_families(families)
    _create_plots(data_points, fit_results, output_img_path)
    _save_fit_results(fit_results, output_fit_results_path)


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


def _deduplicate_and_format_results(results: list[SimulationOutput]) -> list[DataPoint]:
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

    return [
        DataPoint(
            amplitude=result.initial_amplitude,
            bh_mass=result.black_hole_mass,
            formation_time=result.final_simulation_time,
        )
        for result in results
        if result.black_hole_mass is not None
    ]


def _group_into_families(data_points: list[DataPoint]) -> list[Family]:
    """Group data points into families based on formation times (within 1 unit)."""
    if not data_points:
        return []

    # Sort by formation time
    sorted_points = sorted(data_points, key=lambda p: p.formation_time)

    families: list[Family] = []
    current_family: list[DataPoint] = [sorted_points[0]]

    for point in sorted_points[1:]:
        # Check if this point is within 1 unit of any point in current family
        if any(abs(point.formation_time - p.formation_time) <= 1.0 for p in current_family):
            current_family.append(point)
        else:
            # Start a new family
            families.append(_create_family(current_family))
            current_family = [point]

    # Don't forget the last family
    if current_family:
        families.append(_create_family(current_family))

    return sorted(families, key=lambda f: f.min_amplitude)


def _create_family(points: list[DataPoint]) -> Family:
    """Create a Family object from a list of points."""
    amplitudes = [p.amplitude for p in points]
    return Family(
        points=points,
        min_amplitude=min(amplitudes),
        max_amplitude=max(amplitudes),
    )


def _fit_families(families: list[Family]) -> list[FitResult]:
    """Fit power law curves to each family (except the first)."""
    fit_results: list[FitResult] = []

    for i in range(1, len(families)):
        family = families[i]
        prev_family = families[i - 1]

        amp_star_min = prev_family.max_amplitude
        amp_star_max = family.min_amplitude

        fit_points = [p for p in family.points if p.amplitude < amp_star_max * 1.05]

        fit_result = _fit_power_law_with_bootstrap(fit_points, amp_star_min, amp_star_max)
        fit_results.append(fit_result)

    return fit_results


def _fit_power_law_with_bootstrap(
    points: list[DataPoint],
    amp_star_min: float,
    amp_star_max: float,
    n_bootstrap: int = 10000,
) -> FitResult:
    """Fit power law with bootstrapping to estimate uncertainty."""
    amplitudes = np.array([p.amplitude for p in points])
    masses = np.array([p.bh_mass for p in points])
    n_points = len(amplitudes)

    gamma_samples = []
    amp_star_samples = []
    intercept_samples = []
    fitted_masses_all = []

    amp_range_max = float(amplitudes.max())

    for _ in range(n_bootstrap):
        amp_star = RNG.uniform(amp_star_min, amp_star_max)
        amp_star_samples.append(amp_star)

        log_x = np.log(amplitudes - amp_star)
        log_y = np.log(masses)

        design_matrix = np.vstack([log_x, np.ones(n_points)]).T
        coeffs, residuals, _, _ = np.linalg.lstsq(design_matrix, log_y, rcond=None)
        gamma = coeffs[0]
        intercept = coeffs[1]

        # Calculate covariance matrix of coefficients
        mse = residuals[0] / (n_points - 2)
        cov_matrix = mse * np.linalg.inv(design_matrix.T @ design_matrix)

        # Sample from bivariate normal (accounting for within-fit uncertainty and covariance)
        sampled_coeffs = RNG.multivariate_normal([gamma, intercept], cov_matrix)
        gamma_with_noise = sampled_coeffs[0]
        intercept_with_noise = sampled_coeffs[1]

        gamma_samples.append(gamma_with_noise)
        intercept_samples.append(intercept_with_noise)

        # Generate fitted masses for this bootstrap iteration
        amp_range_sample = np.linspace(amp_star, amp_range_max, 100)
        masses_sample = (
            np.exp(intercept_with_noise) * (amp_range_sample - amp_star) ** gamma_with_noise
        )
        fitted_masses_all.append(masses_sample)

    gamma_mean = float(np.mean(gamma_samples))
    amp_star_mean = float(np.mean(amp_star_samples))
    intercept_mean = float(np.mean(intercept_samples))
    gamma_min_95 = float(np.percentile(gamma_samples, 2.5))
    gamma_max_95 = float(np.percentile(gamma_samples, 97.5))

    # Generate fitted curve using mean values
    amp_range = np.linspace(amp_star_mean, amp_range_max, 100)
    fitted_masses_mean = np.exp(intercept_mean) * (amp_range - amp_star_mean) ** gamma_mean

    # Calculate confidence intervals from all bootstrap samples
    fitted_masses_all_array = np.array(fitted_masses_all)
    fitted_masses_lower = np.percentile(fitted_masses_all_array, 2.5, axis=0)
    fitted_masses_upper = np.percentile(fitted_masses_all_array, 97.5, axis=0)

    return FitResult(
        gamma=gamma_mean,
        gamma_min_95=gamma_min_95,
        gamma_max_95=gamma_max_95,
        amp_star=amp_star_mean,
        amp_star_min=amp_star_min,
        amp_star_max=amp_star_max,
        fitted_amplitudes=amp_range,
        fitted_masses=fitted_masses_mean,
        fitted_masses_lower=fitted_masses_lower,
        fitted_masses_upper=fitted_masses_upper,
    )


def _create_plots(
    data_points: list[DataPoint],
    fit_results: list[FitResult],
    output_path: Path,
) -> None:
    """Create the dual plot showing BH Mass and Formation Time vs epsilon."""
    amplitudes = [p.amplitude for p in data_points]
    bh_masses = [p.bh_mass for p in data_points]
    formation_times = [p.formation_time for p in data_points]

    _, (ax1, ax2) = plt.subplots(2, 1, figsize=(10, 8), sharex=True)

    ax1.scatter(amplitudes, bh_masses, s=10, alpha=0.7, color="blue", zorder=3)

    colors = cm.rainbow(np.linspace(0, 1, len(fit_results)))  # type: ignore[attr-defined]
    for fit_result, color in zip(fit_results, colors, strict=False):
        label = rf"$\epsilon$ = {fit_result.amp_star:.3f}; $\gamma$ = {fit_result.gamma:.3f}"
        ax1.plot(
            fit_result.fitted_amplitudes,
            fit_result.fitted_masses,
            color=color,
            linewidth=1,
            label=label,
            zorder=2,
        )

    ax1.set_ylabel("Black Hole Mass")
    ax1.set_ylim(0, max(bh_masses) * 1.1 if bh_masses else 0.02)
    ax1.grid(visible=True, alpha=0.3)
    ax1.legend(loc="upper left", fontsize=8)

    ax2.scatter(amplitudes, formation_times, s=10, alpha=0.7, color="blue")
    ax2.set_xlabel("Initial Amplitude")
    ax2.set_ylabel("Formation Time")
    ax2.set_ylim(0, max(formation_times) * 1.1 if formation_times else 15)
    ax2.grid(visible=True, alpha=0.3)

    min_amp = min(amplitudes) if amplitudes else 18
    max_amp = max(amplitudes) if amplitudes else 38
    ax1.set_xlim(min_amp - 0.5, max_amp + 0.5)
    ax2.set_xlim(min_amp - 0.5, max_amp + 0.5)

    plt.tight_layout()
    plt.savefig(output_path, dpi=300, bbox_inches="tight")


def _save_fit_results(fit_results: list[FitResult], output_path: Path) -> None:
    """Save the fit results to a JSON file."""
    keys = ["gamma", "gamma_min_95", "gamma_max_95", "amp_star", "amp_star_min", "amp_star_max"]
    with Path(output_path).open("w") as f:
        for result in fit_results:
            json.dump({key: getattr(result, key) for key in keys}, f)
            f.write("\n")


if __name__ == "__main__":
    main()
