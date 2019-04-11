[![Build Status](https://travis-ci.org/pchmielowski/Rust-learning.svg?branch=master)](https://travis-ci.org/pchmielowski/Rust-learning)

Steps to build on WSL:

```
sudo apt-get install cmake libxext-dev libsdl2-dev
cargo build
```

For WSL:
1. Start SSH server.
```
sudo apt-get remove openssh-server
sudo apt-get install openssh-server # To make sure the full version is installed
sudo service ssh --full-restart
```

2. Install and run Xming on windows.

3. Set display on WSL
```
export DISPLAY=:0
```
