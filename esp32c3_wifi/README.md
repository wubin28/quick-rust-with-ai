# Blinking the wifi LED on esp32-c3-devkit-rust-1 board using Rust on Ubuntu

## How to run the code

- Purchase the ESP32-C3-DevKit-Rust-1 board
- Connect the board to your computer via USB C cable
- Checking the hardware on Ubuntu
  - Linux: a USB device under lsusb. The device will have a VID (Vendor ID) of 303a and a PID (Product ID) of 1001 -- the 0x prefix will be omitted in the output of lsusb:

    ```sh
    $ lsusb | grep USB
    Bus 003 Device 002: ID 303a:1001 Espressif USB JTAG/serial debug unit
    ```

- Rust toolchain
  - Install nightly Rust and set it as the default toolchain to add support for the target architecture using the following command:

    ```sh
    rustup toolchain list
    rustup show
    rustup toolchain install nightly-2024-06-30 --component rust-src
    rustup default nightly-2024-06-30
    ```
- Espressif toolchain

  - The espflash and cargo-espflash commands assume version is >= 2

    ```sh
    cargo install cargo-espflash espflash ldproxy
    ```

- Toolchain Dependencies on Debian/Ubuntu
  
    ```sh
    sudo apt install llvm-dev libclang-dev clang libudev-dev libuv1-dev pkgconf python3-venv python-is-python3
    ```

- Configure the wifi credentials

    We use toml-cfg throughout this workshop as a more convenient and secure alternative to putting credentials or other sensitive information directly in source code. The settings are stored in a file called cfg.toml in the respective package root instead.

    This configuration contains exactly one section header which has the same name as your package (name = "your-package" in Cargo.toml), and the concrete settings will differ between projects:


    ```toml
    [your-package]
    user = "example"
    password = "h4ckm3"
    ```

    If you copy a cfg.toml to a new project, remember to change the header to [name-of-new-package].

- Build and flash the code to the board

    To test Wi-Fi connectivity, you will have to provide your network name (SSID) and password (PSK). These credentials are stored in a dedicated cfg.toml file (which is .gitignored) to prevent accidental disclosure by sharing source code or doing pull requests. An example is provided.

    Copy cfg.toml.example to cfg.toml (in the same directory) and edit it to reflect your actual credentials:

    ```sh
    $ cp cfg.toml.example cfg.toml
    $ $EDITOR cfg.toml
    $ cat cfg.toml
    ```

    ```toml
    [your-package]
    wifi_ssid = "Your Wifi name"
    wifi_psk = "Your Wifi password"
    ```

    ```sh
    cd bwifi
    cargo build
    cargo run
    ```
