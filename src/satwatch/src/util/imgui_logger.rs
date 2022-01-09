use imgui::*;
use log::{Level, Metadata, Record};
use std::collections::VecDeque;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

fn level_to_color(level: Level) -> imgui::ImColor32 {
    match level {
        Level::Error => imgui::ImColor32::from_rgb(220, 20, 60),
        Level::Warn => imgui::ImColor32::from_rgb(255, 215, 0),
        Level::Info => imgui::ImColor32::from_rgb(255, 255, 255),
        Level::Debug => imgui::ImColor32::from_rgb(123, 104, 238),
        Level::Trace => imgui::ImColor32::from_rgb(169, 169, 169),
    }
}

pub struct ImguiLogger {
    message_tx: Option<SyncSender<(Level, String)>>,
}

pub struct ImguiLoggerUi {
    message_rx: Receiver<(Level, String)>,
    messages: VecDeque<(Level, String)>,
    max_count: usize,
}

impl ImguiLoggerUi {
    pub fn init() -> Self {
        unsafe {
            log::set_logger(&LOGGER).unwrap();
        }
        let (message_tx, message_rx) = sync_channel(1000);
        unsafe {
            LOGGER.message_tx = Some(message_tx);
        }

        log::info!("Imgui Logger Created");
        Self {
            message_rx,
            messages: VecDeque::new(),
            max_count: 1000,
        }
    }
    pub fn draw(&mut self, ui: &mut Ui) {
        // collect and trunkate messages
        'collect_loop: loop {
            match self.message_rx.try_recv() {
                Ok((level, msg)) => self.messages.push_back((level, msg)),
                Err(_) => {
                    break 'collect_loop;
                }
            }
        }
        if self.messages.len() > self.max_count {
            let to_drain = self.messages.len() - self.max_count;
            self.messages.drain(0..to_drain);
        }

        ui.window("Log")
            .save_settings(false)
            .size_constraints([150.0, 300.0], [150000.0, 300000.0])
            .build(|| {
                for (level, msg) in &self.messages {
                    let _stack_token = ui
                        .push_style_color(StyleColor::Text, level_to_color(*level).to_rgba_f32s());
                    ui.text(msg);
                }
            });
    }
}

impl log::Log for ImguiLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if let Some(tx) = self.message_tx.as_ref() {
                let s = format!("{} - {}", record.level(), record.args());
                eprintln!("{}", s);
                tx.send((record.level(), s)).unwrap();
            }
        }
    }

    fn flush(&self) {}
}

static mut LOGGER: ImguiLogger = ImguiLogger { message_tx: None };
