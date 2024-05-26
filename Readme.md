# A i2c playground with esp-rs on core (no-std)

Using the dev board [ESP32-c3-DEVKIT-RUST-1](1) I am training myself on rust for esp32 using the no std variant.

## Target

- Use all the peripherals in the board except wifi and Bluetooth
    - [ ] I2C: [T&H](2)
    - [ ] I2C: [IMU](3)
    - [ ] LED
    - [ ] Push button
    - [ ] Fancy LED (three colours :-D)
    - [ ] ~~Wifi~~  
    - [ ] ~~Bluetooth~~ 

## Challenges so far

The [T&H sensor](https://sensirion.com/media/documents/643F9C8E/63A5A436/Datasheet_SHTC3.pdf) is not acknowledging my messages. I will look into hall driver [suggested](4) by the board repo to use the T&H sensor. Maybe I can learn how to make an embeeded-hal driver for an IC.

[1]: https://github.com/esp-rs/esp-rust-board
[2]: https://sensirion.com/media/documents/643F9C8E/63A5A436/Datasheet_SHTC3.pdf
[3]: https://invensense.tdk.com/download-pdf/icm-42670-p-datasheet/
[4]: https://github.com/esp-rs/esp-rust-board?tab=readme-ov-file#i2c-peripherals
