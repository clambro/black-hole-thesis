cargo build --release
./target/release/black-hole-thesis --level-of-discretization 10
./target/release/black-hole-thesis --level-of-discretization 11
./target/release/black-hole-thesis --level-of-discretization 12
python3 q_factor.py 20_10 20_11 20_12 ingoing
python3 q_factor.py 20_10 20_11 20_12 outgoing
python3 q_factor.py 20_10 20_11 20_12 energy_density
python3 energy_conservation.py 20_10 20_11 20_12
python3 initial_conditions.py 20_12