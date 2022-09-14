use crate::libs::time_util::millis;
use crate::internal_representation::controller_input::ControllerInput;


pub trait Runnable {
    fn run(&mut self) -> ();
}

pub trait TimedRunnable : Runnable {
    fn run_for(&mut self, miliseconds: u32) -> ();
}

pub trait Model: TimedRunnable {
    fn on_input(&mut self, input: ControllerInput) -> ();
}

pub trait View: TimedRunnable {
    fn update(&mut self, game_board: [[u8; 8]; 8]) -> ();

}

pub trait Controller {
    fn read_input(&mut self) -> ControllerInput;
}

impl<T> TimedRunnable for T where T: Runnable {
    fn run_for(&mut self, miliseconds: u32) -> () {
        let time_slice_start = millis();
        let mut current_time = millis();
        while current_time - time_slice_start < miliseconds {
            self.run();
            current_time = millis();
        }
    }
}
