# A template for lighting up an LED on a micro:bit v2 board

## How to generate a new project

**Install Rust:**

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Install cargo-generate:**

```
cargo install cargo-generate
```

If you see the error: "failed to run custom build command for openssl-sys v0.9.104" when installing cargo-generate on Ubuntu, please run the following commands to install openssl dev library:

```
sudo apt update
sudo apt install libssl-dev pkg-config
# verify the installation
pkg-config --modversion openssl
# should see something like 3.0.2
```

**Generate a new project:**

```
cargo generate wubin28/mb2-led-template
```

## How to build and run the code in a native ubuntu

### Build the code

Before cross-compiling you have to download a pre-compiled version of the standard library (a reduced version of it, actually) for your target:

```
rustup target add thumbv7em-none-eabihf
rustup target list --installed
cargo build
```


### Run the code


**Install the [probe-rs](https://probe.rs/docs/getting-started/installation/) tools:**


```
# For Linux, macOS, and Windows
cargo install cargo-binstall
cargo binstall probe-rs-tools

# For macOS if you use brew
brew tap probe-rs/probe-rs
```

**On Ubuntu and macOS:**

```
cargo run
```

**On Windows 11:**
```
cargo run -- --probe <VID:PID:SN>
```

## How to build the code in a docker container and flash it on a micro:bit v2 board on ubuntu

### Build the docker image

```
docker build -t <your-dockerhub-username>/rust-mb2-led .
```

### Push the docker image to dockerhub

```
docker login
docker push <your-dockerhub-username>/rust-mb2-led
```

### Pull the docker image on ubuntu

```
docker pull <your-dockerhub-username>/rust-mb2-led
```

### Run the docker container

```
docker run --rm -it \
    -v $(pwd):/home/user/project \
    -w /home/user/project \
    <your-dockerhub-username>/rust-mb2-led
```
Explanation:

- `--rm`: Removes the container after exit.

- `-it`: Runs in interactive mode.

- `-v $(pwd):/home/user/project`: Mounts your external project directory $(pwd) to internal directory /home/user/project inside the container.

- `-w /home/user/project`: Sets the working directory inside the container.

### Build the code in the docker container

```
cargo clean
cargo build --release
```

The binary will be in the `target/thumbv7em-none-eabihf/release/mem-match` directory.

### Check the binary in the host machine

```
ls target/thumbv7em-none-eabihf/release/mem-match
```

### Flash the binary to the micro:bit v2 board on the host machine

```
probe-rs --version
probe-rs run --chip nRF52833_xxAA target/thumbv7em-none-eabihf/release/mem-match
```

## How to debug the code on Ubuntu

**On terminal 1:**

```
cd c
cargo embed
```

**On terminal 2:**

```
cd mem-match
gdb-multiarch target/thumbv7em-none-eabihf/debug/mem-match
(gdb) target remote :1337
(gdb) break main
(gdb) continue
(gdb) quit
```

## How to debug the code on macOS

**On terminal 1:**

```
cd mem-match
cargo embed
```

**On terminal 2:**

```
cd mem-match
arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/mem-match
(gdb) target remote :1337
(gdb) break main
(gdb) continue
(gdb) quit
```

## How to debug the code on Windows 11

**On PowerShell 1:**

```
cd mem-match
cargo embed --probe <VID:PID:SN>
```

**On PowerShell 2:**

```
cd mem-match
arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/mem-match
(gdb) target remote :1337
(gdb) break main
(gdb) continue
(gdb) quit
```
