cargo build --release
./target/release/black-hole-thesis --level-of-discretization 9
./target/release/black-hole-thesis --level-of-discretization 10
./target/release/black-hole-thesis --level-of-discretization 11
python3 q_factor.py 20_9 20_10 20_11 ingoing
python3 q_factor.py 20_9 20_10 20_11 outgoing
python3 q_factor.py 20_9 20_10 20_11 energy_density
python3 energy_conservation.py 20_9 20_10 20_11
python3 initial_conditions.py 20_11