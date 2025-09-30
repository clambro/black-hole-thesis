cargo build --release
./target/release/black-hole-thesis --level-of-discretization 8
./target/release/black-hole-thesis --level-of-discretization 9
./target/release/black-hole-thesis --level-of-discretization 10
# python3 q_factor.py 20_8 20_9 20_10 ingoing
# python3 q_factor.py 20_8 20_9 20_10 outgoing
# python3 q_factor.py 20_8 20_9 20_10 energy_density
# python3 energy_conservation.py 20_8 20_9 20_10
python3 initial_conditions.py 20_8