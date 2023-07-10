cargo build --release --bin perk_solver_cli
Copy .\target\release\perk_solver_cli.exe .\perk_solver.exe

cargo build --release --bin perk_solver_gui --features="gui"
Copy .\target\release\perk_solver_gui.exe .\perk_solver_gui.exe