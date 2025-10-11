cargo build --release
./target/release/black-hole-thesis --level-of-discretization 11
./target/release/black-hole-thesis --level-of-discretization 12
./target/release/black-hole-thesis --level-of-discretization 13
uv --directory visualization run q_factor.py 30_11 30_12 30_13 field
uv --directory visualization run q_factor.py 30_11 30_12 30_13 conj_momentum
uv --directory visualization run q_factor.py 30_11 30_12 30_13 energy_density
uv --directory visualization run check_convergence.py 30_11 30_12 30_13 --type energy
uv --directory visualization run check_convergence.py 30_11 30_12 30_13 --type mass_residual
