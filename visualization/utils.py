from pathlib import Path

from schemas import SimulationOutput, StateOutput


def load_simulation_output(folder: str) -> SimulationOutput:
    """Load simulation output from a folder."""
    with Path(f"../results/{folder}/simulation_output.json").open() as f:
        return SimulationOutput.model_validate_json(f.read())


def load_state_outputs(folder: str) -> list[StateOutput]:
    """Load state outputs from a folder's states.jsonl file."""
    with Path(f"../results/{folder}/states.jsonl").open() as f:
        return [StateOutput.model_validate_json(line.strip()) for line in f]
