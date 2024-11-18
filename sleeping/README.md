# An embedded Rust project template for the micro:bit v2

## How to generate a new project
```
cargo generate wubin28/rusty_mb2_template_led_roulette
```

## How to run the code

### On Ubuntu and macOS:
```
cargo run
```

### On Windows 11:
```
cargo run --probe <VID:PID:SN>
```

## How to debug the code on Ubuntu

### On terminal 1:
```
cd sleeping
cargo embed
```

### On terminal 2:
```
cd sleeping
gdb-multiarch target/thumbv7em-none-eabihf/debug/sleeping
(gdb) target remote :1337
(gdb) break main
(gdb) continue
(gdb) quit
```

## How to debug the code on macOS

### On terminal 1:
```
cd sleeping
cargo embed
```

### On terminal 2:
```
cd sleeping
arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/sleeping
(gdb) target remote :1337
(gdb) break main
(gdb) continue
(gdb) quit
```

## How to debug the code on Windows 11

### On PowerShell 1:
```
cd sleeping
cargo embed --probe <VID:PID:SN>
```

### On PowerShell 2:
```
cd sleeping
arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/sleeping
(gdb) target remote :1337
(gdb) break main
(gdb) continue
(gdb) quit
```
