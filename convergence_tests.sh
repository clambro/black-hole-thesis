cargo build --release
./target/release/black-hole-thesis --level-of-discretization 11
./target/release/black-hole-thesis --level-of-discretization 12
./target/release/black-hole-thesis --level-of-discretization 13
python3 q_factor.py 30_11 30_12 30_13 radial_gradient
python3 q_factor.py 30_11 30_12 30_13 conj_momentum
python3 q_factor.py 30_11 30_12 30_13 energy_density
python3 energy_conservation.py 30_11 30_12 30_13
python3 compare_mass_calcs.py 30_11 30_12 30_13
python3 initial_conditions.py 30_13
