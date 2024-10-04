use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
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

    values: [[(u8, u8, u8); 8]; 8],

    // default: 64 (or around 1/4 brightness)
    brightness: u8,
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
            values: Default::default(),
            brightness: 64,
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
    fn set_brighness(&mut self, brightness: f32) {
        assert!((0.0..=1.0).contains(&brightness));
        self.brightness = (brightness * 255.0) as u8
    }

    fn update(&mut self, mut f: impl FnMut(usize, usize, &mut (u8, u8, u8))) {
        for (i, row) in self.values.iter_mut().enumerate() {
            for (j, led) in row.iter_mut().enumerate() {
                f(i, j, led)
            }
        }

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

                for (i, row) in self.values.iter().enumerate() {
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

        if event::poll(Duration::from_millis(16)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Release {
                    if matches!(
                        key.code,
                        KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right
                    ) {
                        self.joystick_position = JoystickPosition::Center;
                    }
                    return;
                }
                match key.code {
                    KeyCode::Char('q') => {
                        panic!("nothing to see here, move along");
                    }
                    KeyCode::Up => self.joystick_position = JoystickPosition::Up,
                    KeyCode::Down => self.joystick_position = JoystickPosition::Down,
                    KeyCode::Left => self.joystick_position = JoystickPosition::Left,
                    KeyCode::Right => self.joystick_position = JoystickPosition::Right,
                    _ => {}
                }
            }
        }
    }

    fn get_sin(&self) -> fn(f32) -> f32 {
        f32::sin
    }

    fn joystick_is_up(&mut self) -> bool {
        self.joystick_position == JoystickPosition::Up
    }

    fn joystick_is_down(&mut self) -> bool {
        self.joystick_position == JoystickPosition::Down
    }

    fn joystick_is_left(&mut self) -> bool {
        self.joystick_position == JoystickPosition::Left
    }

    fn joystick_is_right(&mut self) -> bool {
        self.joystick_position == JoystickPosition::Right
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum JoystickPosition {
    Center,
    Up,
    Down,
    Left,
    Right,
}
