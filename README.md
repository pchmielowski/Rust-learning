[![Build Status](https://travis-ci.org/pchmielowski/Rust-learning.svg?branch=master)](https://travis-ci.org/pchmielowski/Rust-learning)

Steps to run on WSL:

1. Install dependencies (see `.travis.yml`, also add `cmake`).

2. Start SSH server.
```
sudo apt-get remove openssh-server
sudo apt-get install openssh-server # To make sure the full version is installed
sudo service ssh --full-restart
```

3. Install and **run** Xming on Windows (host).

4. Set display on WSL
```
export DISPLAY=:0
```

5. Run:
```
cargo run --release
```