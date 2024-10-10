use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use led_matrix_core::{JoystickPosition, HEIGHT, WIDTH};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Position, Rect},
    style::{Color, Stylize},
    text::Text,
    widgets::{Block, Paragraph},
    Terminal,
};

pub struct LedMatrix {
    terminal: Terminal<CrosstermBackend<Stdout>>,

    joystick_position: JoystickPosition,
    joystick_pressed: bool,
    switch: bool,

    leds: [[(u8, u8, u8); WIDTH as usize]; HEIGHT as usize],
}

pub fn run<F: FnOnce(LedMatrix) + Send + 'static>(f: F) -> ! {
    stdout().execute(EnterAlternateScreen).unwrap();
    enable_raw_mode().unwrap();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    terminal.clear().unwrap();

    let matrix = LedMatrix {
        terminal,
        joystick_position: JoystickPosition::Center,
        joystick_pressed: false,
        switch: false,
        leds: Default::default(),
    };

    f(matrix);

    // necessary to make the run function non-terminating
    #[allow(clippy::empty_loop)]
    loop {}
}

impl LedMatrix {
    // Process available events from crossterm and update internal state
    // accordingly. Do this frequently so quitting the app is snappy.
    fn poll_event(&mut self) {
        while let Ok(true) = event::poll(Duration::new(0, 0)) {
            if let event::Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        panic!("nothing to see here, move along");
                    }
                    KeyCode::Up => self.joystick_position = JoystickPosition::Up,
                    KeyCode::Down => self.joystick_position = JoystickPosition::Down,
                    KeyCode::Left => self.joystick_position = JoystickPosition::Left,
                    KeyCode::Right => self.joystick_position = JoystickPosition::Right,
                    KeyCode::Char(' ') => self.joystick_pressed = true,
                    KeyCode::Enter => self.switch = !self.switch,
                    KeyCode::Char('r') => {
                        self.joystick_position = JoystickPosition::Center;
                        self.joystick_pressed = false;
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Drop for LedMatrix {
    fn drop(&mut self) {
        stdout().execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}

impl led_matrix_core::LedMatrixCore for LedMatrix {
    fn apply(&mut self) {
        self.poll_event();

        self.terminal
            .draw(|frame| {
                let size = frame.size();
                if !size.contains(Position::new(15, 8)) {
                    frame.render_widget(Paragraph::new("terminal is too small"), size);
                    return;
                }
                let pixel_size = match () {
                    _ if size.contains(Position::new(79, 41)) => 5,
                    _ if size.contains(Position::new(63, 33)) => 4,
                    _ if size.contains(Position::new(47, 25)) => 3,
                    _ if size.contains(Position::new(31, 17)) => 2,
                    _ => 1,
                };

                let area = Rect::new(0, 0, size.width, 2);
                frame.render_widget(
                    Text::raw("joystick: move: arrows, press: space, release: space\nswitch: enter, quit: Q"),
                    area,
                );
                for (i, row) in self.leds.iter().enumerate() {
                    for (j, led) in row.iter().enumerate() {
                        let area = Rect::new(
                            j as u16 * 2 * pixel_size,
                            i as u16 * pixel_size + 2, // + 2 bcs of help text
                            2 * pixel_size,
                            pixel_size,
                        );
                        let (r, g, b) = *led;
                        let color = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                        frame.render_widget(Block::new().bg(Color::from_u32(color)), area);
                    }
                }
            })
            .unwrap();

        self.poll_event();
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
        self.poll_event();
        self.switch
    }

    fn joystick_pressed(&mut self) -> bool {
        self.poll_event();
        self.joystick_pressed
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
