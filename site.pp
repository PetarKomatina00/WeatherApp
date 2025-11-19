sudo apt install -y curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
export PATH="$HOME/.cargo/bin:$PATH"
sudo apt install -y postgresql
sudo apt install -y redis-server
