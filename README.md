# Black Hole Formation Simulation

This is a redo of my undergraduate thesis in Rust. Briefly and non-technically, we are simulating the evolution of a pulse of energy in a confined region of space. If the pulse has enough energy to begin with, it will immediately collapse into a black hole (thus ending the simulation). If it does not have enough energy, it will start dispersing out to the boundary of our simulation, at which point it will be redirected back towards the center. The idea is that every time the pulse collapses through the center, its own self-gravity focuses it to be denser and denser, until it inevtiably forms a black hole.

A detailed (and not too technical) explanation of this process is provided here (link here when ready). Here is a visualization of the energy focusing under several implosions and forming a black hole:

(add gif for reflection)

Here is the pattern of black hole mass and formation time vs initial pulse amplitude:

(add final graph)

## Project Structure

- `src/` - The main simulation code in Rust.
- `visualization/` - A small Python library for visualizing the results of the simulation.
- `docs/` - Accompanying documentation for the project.


## Running the Simulation

```bash
# Build the project
cargo build

# Run the default simulation
cargo run

# Run with custom parameters
cargo run -- --amplitude 50.0 --max-time 20.0
```

### Command Line Parameters

- `--level-of-discretization` (default: 15) - Spatial resolution of the simulation grid
- `--amplitude` (default: 30.0) - Initial strength of the energy perturbation
- `--output-dt` (default: 0.0067) - Time between output frames (affects animation smoothness)
- `--output-dx-level` (default: 9) - Resolution for output data
- `--max-time` (default: 15.0) - Maximum simulation time before timeout
- `--skip-state-output` (default: false) - Skip the temporal state output to save space
