use curl::easy::*;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;
use std::sync::mpsc::*;

pub struct DownloadProgress {
    pub dl_total: f64,
    pub dl_now: f64,
}

pub enum DownloadStatus {
    Invalid,
    Progress(DownloadProgress),
    Error(String),
    FileComplete(PathBuf),
}

pub struct Downloader {
    rx: Receiver<DownloadStatus>,
    last_status: DownloadStatus,
}

impl Downloader {
    pub fn download_to_file(url: &str, path: PathBuf) -> Self {
        let (tx, rx) = sync_channel(10);
        let url = url.to_owned();
        let path_copy = path.clone();
        std::thread::spawn(move || {
            let tmp_path = if path_copy.extension().is_some() {
                let mut p = path_copy.clone();
                p.set_extension(format!(
                    "{}{}",
                    p.extension().unwrap().to_str().unwrap(),
                    ".part"
                ));
                p
            } else {
                let mut p = path_copy.clone();
                p.set_extension("part");
                p
            };
            match File::create(tmp_path.clone()) {
                Ok(mut file) => {
                    let mut easy = Easy::new();
                    easy.url(&url);
                    let progress_tx = tx.clone();
                    easy.progress(true);
                    easy.progress_function(move |total, now, _, _| {
                        progress_tx.send(DownloadStatus::Progress(DownloadProgress {
                            dl_total: total,
                            dl_now: now,
                        }));
                        true
                    });
                    easy.write_function(move |data| {
                        file.write(data);
                        Ok(data.len())
                    });
                    match easy.perform() {
                        Ok(_) => {
                            std::fs::rename(tmp_path, path_copy.clone());
                            tx.send(DownloadStatus::FileComplete(path_copy));
                        }
                        Err(e) => {
                            std::fs::remove_file(tmp_path);
                            tx.send(DownloadStatus::Error(e.to_string()));
                        }
                    }
                }
                Err(e) => {
                    tx.send(DownloadStatus::Error(e.to_string()));
                }
            }
        });

        Self {
            rx,
            last_status: DownloadStatus::Invalid,
        }
    }

    pub fn update_status(&mut self) {
        'status_loop: loop {
            match self.rx.try_recv() {
                Ok(new_status) => self.last_status = new_status,
                Err(_) => {
                    break 'status_loop;
                }
            }
        }
    }

    pub fn get_status(&self) -> &DownloadStatus {
        &self.last_status
    }
}
