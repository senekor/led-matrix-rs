use std::sync::mpsc::{self, Receiver, Sender};

use led_matrix_core::{JoystickPosition, HEIGHT, WIDTH};
use serde::{Deserialize, Serialize};

mod gui;

pub type LedGrid = [[(u8, u8, u8); WIDTH as usize]; HEIGHT as usize];

pub struct LedMatrix {
    sender: Sender<LedGrid>,
    receiver: Receiver<Event>,

    joystick_position: JoystickPosition,
    switch: bool,

    leds: [[(u8, u8, u8); WIDTH as usize]; HEIGHT as usize],
}

pub fn run<F: FnOnce(LedMatrix) + Send + 'static>(f: F) -> ! {
    let (event_sender, event_receiver) = mpsc::channel();
    let (led_grid_sender, led_grid_receiver) = mpsc::channel();

    let matrix = LedMatrix {
        sender: led_grid_sender,
        receiver: event_receiver,
        joystick_position: Default::default(),
        switch: Default::default(),
        leds: Default::default(),
    };

    std::thread::spawn(move || f(matrix));

    gui::run(event_sender, led_grid_receiver);

    // necessary to make the run function non-terminating
    #[allow(clippy::empty_loop)]
    loop {}
}

impl LedMatrix {
    fn poll_event(&mut self) {
        while let Ok(event) = self.receiver.try_recv() {
            use EventKey as K;
            use EventKind::*;
            match event {
                Event { kind: U, key: K::U } => self.joystick_position = JoystickPosition::Up,
                Event { kind: U, key: K::D } => self.joystick_position = JoystickPosition::Down,
                Event { kind: U, key: K::L } => self.joystick_position = JoystickPosition::Left,
                Event { kind: U, key: K::R } => self.joystick_position = JoystickPosition::Right,
                Event { kind: U, key: K::S } => self.switch = !self.switch,
                Event { kind: D, key: K::U } => self.joystick_position = JoystickPosition::Center,
                Event { kind: D, key: K::D } => self.joystick_position = JoystickPosition::Center,
                Event { kind: D, key: K::L } => self.joystick_position = JoystickPosition::Center,
                Event { kind: D, key: K::R } => self.joystick_position = JoystickPosition::Center,
                Event { kind: D, key: K::S } => {}
            };
        }
    }
}

impl led_matrix_core::LedMatrixCore for LedMatrix {
    fn apply(&mut self) {
        self.sender.send(self.leds).unwrap();
    }

    fn set_brightness(&mut self, _brightness: u8) {}

    fn sleep_ms(&mut self, duration: u32) {
        self.poll_event();
        std::thread::sleep(std::time::Duration::from_millis(duration.into()));
        self.poll_event();
    }

    fn get_sin(&self) -> fn(f32) -> f32 {
        f32::sin
    }

    fn joystick_position(&mut self) -> JoystickPosition {
        self.poll_event();
        self.joystick_position
    }

    fn switch(&mut self) -> bool {
        self.switch
    }
}

impl core::ops::Index<(usize, usize)> for LedMatrix {
    type Output = (u8, u8, u8);

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!((0..HEIGHT as usize).contains(&x));
        assert!((0..WIDTH as usize).contains(&y));
        &self.leds[HEIGHT as usize - y - 1][x]
    }
}
impl core::ops::IndexMut<(usize, usize)> for LedMatrix {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        assert!((0..HEIGHT as usize).contains(&x));
        assert!((0..WIDTH as usize).contains(&y));
        &mut self.leds[HEIGHT as usize - y - 1][x]
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventKind {
    U,
    D,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum EventKey {
    U,
    D,
    L,
    R,
    S,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub kind: EventKind,
    pub key: EventKey,
}
