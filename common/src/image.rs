use base64::{engine::general_purpose as b64, Engine as _};
use infer::Infer;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use tauri_icns::IconFamily;

pub struct ImageReader {
    infer: Infer,
}

impl ImageReader {
    pub fn new() -> Self {
        let mut infer = Infer::new();
        infer.add("image/x-icns", "icns", icns_matcher);
        Self { infer }
    }

    pub fn read_data(&self, path: &PathBuf) -> std::io::Result<String> {
        let kind = self
            .infer
            .get_from_path(path)?
            .unwrap_or_else(|| panic!("Unrecogized file type: {}", path.to_string_lossy()));
        let (content, mime): (Vec<u8>, String) = match kind.extension() {
            "icns" => {
                let file = BufReader::new(File::open(path)?);
                let icon_family = IconFamily::read(file)?;
                let best_quality_type = *icon_family
                    .available_icons()
                    .iter()
                    .max_by_key(|icon_type| icon_type.pixel_height())
                    .expect(".icns file does not contain any png images");
                let content = if let Ok(icon) = icon_family.get_icon_with_type(best_quality_type) {
                    icon.into_data().into()
                } else {
                    include_bytes!("../../assets/placeholder-app-icon.png").to_vec()
                };
                (content, "image/png".into())
            }
            _ => {
                let content = std::fs::read(path)?;
                (content, kind.mime_type().into())
            }
        };

        Ok(format!(
            "data:{};base64,{}",
            &mime,
            b64::STANDARD_NO_PAD.encode(content)
        ))
    }
}

impl Default for ImageReader {
    fn default() -> Self {
        Self::new()
    }
}

fn icns_matcher(buf: &[u8]) -> bool {
    buf.len() >= 4 && buf[..4] == [0x69, 0x63, 0x6e, 0x73]
}
