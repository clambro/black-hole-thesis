from pydantic import BaseModel


class SimulationOutput(BaseModel):
    """The final results of a simulation run."""

    initial_amplitude: float
    grid_level: int
    time_taken_seconds: float
    num_steps: int
    final_simulation_time: float
    black_hole_mass: float | None = None


class StateOutput(BaseModel):
    """Serialized state output from the simulation."""

    time: float
    field: list[float]
    radial_gradient: list[float]
    conj_momentum: list[float]
    mass: list[float]
    radial_factor: list[float]
    lapse: list[float]
    char_speed: list[float]
    energy_density: list[float]
    total_energy: float
    alternate_mass: list[float]
