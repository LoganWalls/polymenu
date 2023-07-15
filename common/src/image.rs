use base64::{engine::general_purpose as b64, Engine as _};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use tauri_icns::IconFamily;

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageData {
    pub content: Vec<u8>,
    pub mime: String,
    pub path: String,
}

impl ImageData {
    pub fn from_path(path: &PathBuf) -> std::io::Result<Self> {
        let extension = path
            .extension()
            .expect("Image files must include an extension")
            .to_str()
            .unwrap();
        let (content, mime) = match extension {
            "icns" => {
                let file = BufReader::new(File::open(path)?);
                let icon_family = IconFamily::read(file)?;
                let best_quality_type = *icon_family
                    .available_icons()
                    .iter()
                    .max_by_key(|icon_type| icon_type.pixel_height())
                    .expect(".icns file does not contain any png images");
                let content = icon_family
                    .get_icon_with_type(best_quality_type)?
                    .into_data()
                    .into();
                (content, "png".into())
            }
            ext => {
                let content = std::fs::read(path)?;
                let mime = match ext {
                    "ico" => "x-icon",
                    e => e,
                }
                .into();
                (content, mime)
            }
        };

        Ok(ImageData {
            content,
            mime,
            path: path.to_str().unwrap().into(),
        })
    }

    pub fn b64_content_string(&self) -> String {
        let content = b64::STANDARD_NO_PAD.encode(&self.content);
        format!("data:image/{};base64,{}", &self.mime, &content)
    }
}
