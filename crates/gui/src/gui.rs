use std::sync::mpsc::{Receiver, Sender};

use eframe::egui::{self, Color32, Key, Pos2, Rect, Rounding, Shape};
use led_matrix_core::{HEIGHT, WIDTH};

use crate::{Event, EventKey, EventKind, LedGrid};

pub fn run(sender: Sender<Event>, receiver: Receiver<LedGrid>) {
    let app = LedMatrixApp {
        leds: Default::default(),
        sender,
        receiver,
    };

    eframe::run_native(
        "LED-matrix emulator",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(app))),
    )
    .unwrap();
}

struct LedMatrixApp {
    leds: LedGrid,
    sender: Sender<Event>,
    receiver: Receiver<LedGrid>,
}

impl eframe::App for LedMatrixApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            'rising_edge: {
                let key = if ctx.input(|i| i.key_pressed(Key::ArrowUp)) {
                    EventKey::U
                } else if ctx.input(|i| i.key_pressed(Key::ArrowDown)) {
                    EventKey::D
                } else if ctx.input(|i| i.key_pressed(Key::ArrowLeft)) {
                    EventKey::L
                } else if ctx.input(|i| i.key_pressed(Key::ArrowRight)) {
                    EventKey::R
                } else if ctx.input(|i| i.key_pressed(Key::Space)) {
                    EventKey::P
                } else if ctx.input(|i| i.key_pressed(Key::Enter)) {
                    EventKey::S
                } else if ctx.input(|i| i.key_pressed(Key::Q)) {
                    std::process::exit(0)
                } else {
                    break 'rising_edge;
                };
                let event = Event {
                    kind: EventKind::U,
                    key,
                };
                self.sender.send(event).unwrap();
            }
            'falling_edge: {
                let key = if ctx.input(|i| i.key_released(Key::ArrowUp)) {
                    EventKey::U
                } else if ctx.input(|i| i.key_released(Key::ArrowDown)) {
                    EventKey::D
                } else if ctx.input(|i| i.key_released(Key::ArrowLeft)) {
                    EventKey::L
                } else if ctx.input(|i| i.key_released(Key::ArrowRight)) {
                    EventKey::R
                } else if ctx.input(|i| i.key_released(Key::Space)) {
                    EventKey::P
                } else {
                    break 'falling_edge;
                };
                let event = Event {
                    kind: EventKind::D,
                    key,
                };
                self.sender.send(event).unwrap();
            }

            // drain queue to get the most recent frame
            while let Ok(new_leds) = self.receiver.try_recv() {
                self.leds = new_leds;
            }

            ui.heading("control with arrow keys (joystick), space (joystick press) and enter (switch). Quit with Q.");

            let painter = ui.painter();
            let padding_top = 80.0;
            let r = painter.clip_rect().with_min_y(padding_top);
            let led_size = r.width().min(r.height()) / 8.0;
            for x in 0..WIDTH as usize {
                for y in 0..HEIGHT as usize {
                    let (r, g, b) = self.leds[y][x];
                    let color = Color32::from_rgb(r, g, b);
                    let x = x as f32 * led_size;
                    let y = y as f32 * led_size + padding_top;
                    painter.add(Shape::rect_filled(
                        Rect::from_two_pos(Pos2::new(x, y), Pos2::new(x + led_size, y + led_size)),
                        Rounding::ZERO,
                        color,
                    ));
                }
            }
        });
    }
}
