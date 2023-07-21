mod config;
mod image;
pub mod item;
pub mod keybinds;
pub use clap::Parser;
pub use config::*;
pub use image::ImageReader;
pub use polymenu_derive::UpdateFromOther;

pub trait UpdateFromOther {
    fn update_from_other(&mut self, other: Self);
}
