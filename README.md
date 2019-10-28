[![Build Status](https://travis-ci.com/Crate3D/closed_loop_firmware_demo.svg?branch=master)](https://travis-ci.com/Crate3D/closed_loop_firmware_demo)
# Closed loop demo firmware
Demo version of closed loop servo firmware runned on stm32f4 board with stm32f412 mcu

Build for main development board:
```shell
cargo build --features f412
```

Build for another development board(based on stm32f103 mcu, known as `Blue Pill`):
```shell
cargo build --features f103
```
