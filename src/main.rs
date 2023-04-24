#![no_std]
#![no_main]

use adafruit_seesaw::{devices::NeoKey1x4, prelude::*, SeesawSingleThread};
use defmt::{self, info};
use defmt_rtt as _;
use fugit::RateExtU32;
use panic_probe as _;
use rp_pico::{
    entry,
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac,
        sio::Sio,
        watchdog::Watchdog,
        I2C,
    },
};

const OFF_COLOUR: (u8, u8, u8) = (0, 0, 0);
const ON_COLOUR: (u8, u8, u8) = (200, 200, 255);

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let i2c = I2C::i2c1(
        pac.I2C1,
        pins.gpio26.into_mode(), // sda
        pins.gpio27.into_mode(), // scl
        100.kHz(),
        &mut pac.RESETS,
        125_000_000.Hz(),
    );

    let seesaw = SeesawSingleThread::new(delay, i2c);
    let mut neokeys = NeoKey1x4::new_with_default_addr(seesaw.acquire_driver())
        .init()
        .expect("Failed to start NeoKey1x4");

    loop {
        let keys = neokeys.keys().expect("Failed to read keys");

        if (keys >> 0) & 1 == 0 {
            neokeys.set_nth_neopixel_color(0, ON_COLOUR.0, ON_COLOUR.1, ON_COLOUR.2).unwrap();
        }
        else {
            neokeys.set_nth_neopixel_color(0, OFF_COLOUR.0, OFF_COLOUR.1, OFF_COLOUR.2).unwrap();
        }
        if (keys >> 1) & 1 == 0 {
            neokeys.set_nth_neopixel_color(1, ON_COLOUR.0, ON_COLOUR.1, ON_COLOUR.2).unwrap();
        }
        else {
            neokeys.set_nth_neopixel_color(1, OFF_COLOUR.0, OFF_COLOUR.1, OFF_COLOUR.2).unwrap();
        }
        if (keys >> 2) & 1 == 0 {
            neokeys.set_nth_neopixel_color(2, ON_COLOUR.0, ON_COLOUR.1, ON_COLOUR.2).unwrap();
        }
        else {
            neokeys.set_nth_neopixel_color(2, OFF_COLOUR.0, OFF_COLOUR.1, OFF_COLOUR.2).unwrap();
        }
        if (keys >> 3) & 1 == 0 {
            neokeys.set_nth_neopixel_color(3, ON_COLOUR.0, ON_COLOUR.1, ON_COLOUR.2).unwrap();
        }
        else {
            neokeys.set_nth_neopixel_color(3, OFF_COLOUR.0, OFF_COLOUR.1, OFF_COLOUR.2).unwrap();
        }

        neokeys.sync_neopixel()
            .expect("Failed to update neopixels");
    }
}
