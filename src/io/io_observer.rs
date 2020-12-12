use std::path;
use std::thread;

use crate::io::{FileOp, IOWorkerProgress};
use crate::util::format;

#[derive(Debug)]
pub struct IOWorkerObserver {
    pub handle: thread::JoinHandle<()>,
    pub progress: Option<IOWorkerProgress>,
    msg: String,
    src: path::PathBuf,
    dest: path::PathBuf,
}

impl IOWorkerObserver {
    pub fn new(handle: thread::JoinHandle<()>, src: path::PathBuf, dest: path::PathBuf) -> Self {
        Self {
            handle,
            progress: None,
            src,
            dest,
            msg: String::new(),
        }
    }

    pub fn join(self) -> bool {
        match self.handle.join() {
            Ok(_) => true,
            _ => false,
        }
    }
    pub fn set_progress(&mut self, progress: IOWorkerProgress) {
        self.progress = Some(progress);
    }
    pub fn update_msg(&mut self) {
        match self.progress.as_ref() {
            None => {}
            Some(progress) => {
                let size_str = format::file_size_to_string(progress.processed());
                match progress.kind() {
                    FileOp::Cut => {
                        let msg = format!(
                            "moving ({}/{}) {} completed",
                            progress.index() + 1,
                            progress.len(),
                            size_str
                        );
                        self.msg = msg;
                    }
                    FileOp::Copy => {
                        let msg = format!(
                            "copying ({}/{}) {} completed",
                            progress.index() + 1,
                            progress.len(),
                            size_str
                        );
                        self.msg = msg;
                    }
                }
            }
        }
    }
    pub fn get_msg(&self) -> &str {
        self.msg.as_str()
    }
    pub fn src_path(&self) -> &path::Path {
        self.src.as_path()
    }
    pub fn dest_path(&self) -> &path::Path {
        self.dest.as_path()
    }
}
