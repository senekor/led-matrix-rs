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
    widgets::{Block, Paragraph},
    Terminal,
};

pub struct LedMatrix {
    terminal: Terminal<CrosstermBackend<Stdout>>,

    joystick_position: JoystickPosition,

    leds: [[(u8, u8, u8); WIDTH as usize]; HEIGHT as usize],
}

impl LedMatrix {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        stdout().execute(EnterAlternateScreen).unwrap();
        enable_raw_mode().unwrap();
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        terminal.clear().unwrap();

        Self {
            terminal,
            joystick_position: JoystickPosition::Center,
            leds: Default::default(),
        }
    }

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
                    KeyCode::Char(' ') => self.joystick_position = JoystickPosition::Center,
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

impl led_matrix_core::LedMatrix for LedMatrix {
    fn apply(&mut self) {
        self.poll_event();

        self.terminal
            .draw(|frame| {
                let size = frame.size();
                if !size.contains(Position::new(15, 7)) {
                    frame.render_widget(Paragraph::new("terminal is too small"), size);
                    return;
                }
                let pixel_size = match () {
                    _ if size.contains(Position::new(79, 39)) => 5,
                    _ if size.contains(Position::new(63, 31)) => 4,
                    _ if size.contains(Position::new(47, 23)) => 3,
                    _ if size.contains(Position::new(31, 15)) => 2,
                    _ => 1,
                };

                for (i, row) in self.leds.iter().enumerate() {
                    for (j, led) in row.iter().enumerate() {
                        let area = Rect::new(
                            j as u16 * 2 * pixel_size,
                            i as u16 * pixel_size,
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

    fn set_brighness(&mut self, _brightness: u8) {}

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
}

impl core::ops::Index<(usize, usize)> for LedMatrix {
    type Output = (u8, u8, u8);

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        assert!((0..HEIGHT as usize).contains(&row));
        assert!((0..WIDTH as usize).contains(&col));
        &self.leds[row][col]
    }
}
impl core::ops::IndexMut<(usize, usize)> for LedMatrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        assert!((0..HEIGHT as usize).contains(&row));
        assert!((0..WIDTH as usize).contains(&col));
        &mut self.leds[row][col]
    }
}
