use crate::util::input_events::Event;
use crate::world::world_ui::WorldUi;
use glow::Context;
use imgui::Ui;
use legion::World;
use libspace::timebase::{Timebase, SPICE_LSK_URL};

use crate::download::downloader::*;
use crate::download::DownloadStatus;
use crate::main_loop::app_phase::AppPhase;
use std::path::PathBuf;

struct Download {
    display_name: String,
    downloader: Downloader,
}

pub struct DownloaderUi {
    visible: bool,
    downloads: Vec<Download>,
}

impl DownloaderUi {
    pub fn new() -> Self {
        Self {
            visible: true,
            downloads: Vec::new(),
        }
    }

    pub fn add_file_download(&mut self, name: &str, url: &str, path: PathBuf) {
        let downloader = Downloader::download_to_file(url, path);
        self.downloads.push(Download {
            display_name: name.to_owned(),
            downloader,
        })
    }
}

impl WorldUi for DownloaderUi {
    fn main_menu(&mut self, _ui: &Ui) {}

    fn ui(&mut self, _gl: &Context, _world: &mut World, ui: &mut Ui) -> Result<(), String> {
        let mut visible = self.visible;
        ui.window("Download SPICE Data").build(|| {
            ui.text("Leap Second Kernel");
            ui.same_line();
            if ui.button("Download LSK") {
                self.add_file_download("Test Download", SPICE_LSK_URL, Timebase::lsk_file());
            }
            ui.separator();
            for dl in &mut self.downloads {
                let status = dl.downloader.get_status();
                match status {
                    DownloadStatus::Progress(progress) => {
                        let frac_progress = (progress.dl_now / progress.dl_total) as f32;
                        imgui::ProgressBar::new(frac_progress)
                            .overlay_text(&dl.display_name)
                            .build(ui);
                    }
                    _ => {}
                }
            }
            ui.separator();
            if Timebase::lsk_file().exists() {
                if ui.button("Close and Start") {
                    visible = false;
                }
            }
        });

        self.visible = visible;
        Ok(())
    }

    fn handle_input(&mut self, _gl: &Context, _world: &mut World, _event: Event) {}

    fn tick(
        &mut self,
        _gl: &Context,
        _world: &mut World,
        _timebase: &mut Timebase,
    ) -> Result<(), String> {
        Ok(())
    }

    fn has_global_tick(&self) -> bool {
        true
    }

    fn global_tick(&mut self, _gl: &Context, _world: &mut World) -> Result<AppPhase, String> {
        self.downloads
            .iter_mut()
            .for_each(|dl| dl.downloader.update_status());

        self.downloads.retain(|dl| {
            let status = dl.downloader.get_status();
            match status {
                DownloadStatus::FileComplete(_) => {
                    log::info!("Download complete");
                    false
                }
                DownloadStatus::Error(e) => {
                    log::error!("{}", e);
                    false
                }
                _ => true,
            }
        });
        if self.visible {
            Ok(AppPhase::Downloads)
        } else {
            Ok(AppPhase::Running)
        }
    }
}
