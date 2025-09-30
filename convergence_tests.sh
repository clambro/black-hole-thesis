cargo build --release
./target/release/black-hole-thesis --level-of-discretization 8
./target/release/black-hole-thesis --level-of-discretization 9
./target/release/black-hole-thesis --level-of-discretization 10
python3 q_factor.py 1_8 1_9 1_10 displacement && python3 q_factor.py 1_8 1_9 1_10 momentum && python3 q_factor.py 1_8 1_9 1_10 energy_density
python3 energy_conservation.py 1_8 1_9 1_10