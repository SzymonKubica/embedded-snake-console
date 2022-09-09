use arduino_hal::port::Pin;
use arduino_hal::port::mode::Output;
use arduino_hal::hal::port::{PB0, PB1, PB2, PB3, PB4};
use arduino_hal::hal::port::{PD2, PD3, PD4, PD5, PD6, PD7};
use embedded_hal::digital::v2::OutputPin;
use crate::common::{SNAKE_SEGMENT, APPLE};
use crate::{common, time_util};
use crate::shift_register::ShiftRegister;
use crate::{mvc::{View, Task}, game_engine};

pub struct GameView {
    screen: [[u8; 8]; 8],
    shift_register: ShiftRegister<
        Pin<Output, PB2>,
        Pin<Output, PB3>,
        Pin<Output, PB4>,
        >,
    ground_pins: GroundPins
}

impl GameView {
    pub fn new() -> GameView {
        let peripherals = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(peripherals);

        let clock_pin = pins.d10.into_output();
        let latch_pin = pins.d11.into_output();
        let data_pin = pins.d12.into_output();

        GameView {
            screen: game_engine::initialize_board(),
            shift_register: ShiftRegister::new(clock_pin, latch_pin, data_pin),
            ground_pins: GroundPins::new(),
        }
    }
}

impl View for GameView {
    fn update(&mut self, game_board: [[u8; 8]; 8]) -> () {
        self.screen = game_board;
    }

}

impl Task for GameView {
    fn run(&mut self) -> () {
        let mut outputs = self.shift_register.decompose();
        for i in 0..8_usize {
            outputs[i].set_high(); // Add voltage to the ith row of the matrix


            // Iterate over the row and light up if snake/apple present
            // by setting the corresponding ground pin to low thereby completing
            // the circuit.
            for j in 0..8_usize {
                let current_pixel = self.screen[j][i];
                if current_pixel == SNAKE_SEGMENT || current_pixel == APPLE {
                   self.ground_pins.set_pin_low(j)
                }
            }

            time_util::sleep_ms(common::SCORE_DISPLAY_TIME);
            self.ground_pins.disconnect_ground();
        }
    }
}


struct GroundPins {
    ground_0: Pin<Output, PD2>,
    ground_1: Pin<Output, PD3>,
    ground_2: Pin<Output, PD4>,
    ground_3: Pin<Output, PD5>,
    ground_4: Pin<Output, PD6>,
    ground_5: Pin<Output, PD7>,
    ground_6: Pin<Output, PB0>,
    ground_7: Pin<Output, PB1>
}

impl GroundPins {
    pub fn new() -> GroundPins {
        let peripherals = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(peripherals);

        GroundPins {
            ground_0: pins.d2.into_output_high(),
            ground_1: pins.d3.into_output_high(),
            ground_2: pins.d4.into_output_high(),
            ground_3: pins.d5.into_output_high(),
            ground_4: pins.d6.into_output_high(),
            ground_5: pins.d7.into_output_high(),
            ground_6: pins.d8.into_output_high(),
            ground_7: pins.d9.into_output_high()
        }
    }

    pub fn set_pin_low(&mut self, index: usize) {
        match index {
            0 => self.ground_0.set_low(),
            1 => self.ground_1.set_low(),
            2 => self.ground_2.set_low(),
            3 => self.ground_3.set_low(),
            4 => self.ground_4.set_low(),
            5 => self.ground_5.set_low(),
            6 => self.ground_6.set_low(),
            7 => self.ground_7.set_low(),
            _ => ()
        }
    }

    fn disconnect_ground(&mut self) {
        for i in 0..8usize {
            self.set_pin_high(i);
        }
    }

    fn set_pin_high(&mut self, index: usize) {
        match index {
            0 => self.ground_0.set_high(),
            1 => self.ground_1.set_high(),
            2 => self.ground_2.set_high(),
            3 => self.ground_3.set_high(),
            4 => self.ground_4.set_high(),
            5 => self.ground_5.set_high(),
            6 => self.ground_6.set_high(),
            7 => self.ground_7.set_high(),
            _ => ()
        }
    }

}
