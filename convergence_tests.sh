cargo build --release
./target/release/black-hole-thesis --level-of-discretization 12
./target/release/black-hole-thesis --level-of-discretization 13
./target/release/black-hole-thesis --level-of-discretization 14
uv --directory visualization run q_factor.py 30_12 30_13 30_14 field
uv --directory visualization run q_factor.py 30_12 30_13 30_14 conj_momentum
uv --directory visualization run q_factor.py 30_12 30_13 30_14 energy_density
uv --directory visualization run check_convergence.py 30_12 30_13 30_14 --type energy
uv --directory visualization run check_convergence.py 30_12 30_13 30_14 --type mass_residual
