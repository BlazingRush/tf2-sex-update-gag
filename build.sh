curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
mkdir build
cd build
rustc ../update.rs
echo "built TF2 secret sex update installer!"

