sudo docker run -it --mount type=bind,source=$(pwd)/code,target=/build sunflower:dev /bin/bash -c "cargo build --release --target wasm32-unknown-unknown && wasm-bindgen --out-name sunflower --out-dir pkg --target web target/wasm32-unknown-unknown/release/sunflower_picking.wasm"
sudo docker build -f docker/Dockerfile.serve -t sunflower:serve code/
sudo docker run -p 80:80 sunflower:serve