use base64::{engine::general_purpose as b64, Engine as _};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::OnceLock;
use tauri_icns::{Encoding, IconFamily};

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageData {
    pub content: Vec<u8>,
    pub mime: String,
}

fn icns_matcher(buf: &[u8]) -> bool {
    buf.len() >= 4 && buf[..4] == [0x69, 0x63, 0x6e, 0x73]
}

pub const PLACEHOLDER_IMG: &[u8] = include_bytes!("../../assets/placeholder-app-icon.png");
pub static PLACEHOLDER_IMG_STRING: OnceLock<String> = OnceLock::new();
pub static INFER: OnceLock<infer::Infer> = OnceLock::new();

impl ImageData {
    pub fn from_path(path: &PathBuf) -> std::io::Result<Self> {
        dbg!(&path);
        let kind = INFER
            .get_or_init(|| {
                let mut infer = infer::Infer::new();
                infer.add("image/x-icns", "icns", icns_matcher);
                infer
            })
            .get_from_path(path)?
            .unwrap_or_else(|| panic!("Unrecogized file type: {}", path.to_string_lossy()));
        let (content, mime) = match kind.extension() {
            "icns" => {
                let file = BufReader::new(File::open(path)?);
                let icon_family = IconFamily::read(file)?;
                let icon_type = icon_family
                    .available_icons()
                    .into_iter()
                    .filter(|&icon_type| {
                        icon_type.encoding() == Encoding::JP2PNG && icon_type.pixel_width() < 512
                    })
                    .max_by_key(|&icon_type| icon_type.pixel_width())
                    .expect(".icns file does not contain any png images");
                let content = if let Ok(icon) = icon_family.get_icon_with_type(icon_type) {
                    icon.into_data().into()
                } else {
                    PLACEHOLDER_IMG.to_vec()
                };
                (content, "image/png".into())
            }
            _ => {
                let content = std::fs::read(path)?;
                (content, kind.mime_type().into())
            }
        };

        Ok(ImageData { content, mime })
    }

    pub fn b64_content_string(&self) -> String {
        let content = b64::STANDARD_NO_PAD.encode(&self.content);
        format!("data:{};base64,{}", &self.mime, &content)
    }
}
