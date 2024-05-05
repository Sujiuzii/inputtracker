mod tracker;
mod gui;

const AUDIO_FILE: &[u8] = include_bytes!("../assets/o-pao.mp3");

pub use tracker::{run, check_router};
pub use gui::application;