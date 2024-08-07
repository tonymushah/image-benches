use std::{fs::create_dir_all, path::PathBuf};

use image::ImageFormat;

pub struct ConvertToParam {
    pub file: PathBuf,
    pub output_dir: PathBuf,
    pub format: ImageFormat,
}

impl ConvertToParam {
    fn get_output_dir(&self, create_dirs: bool) -> PathBuf {
        let path = self.output_dir.join(self.format.extensions_str()[0]);
        if !path.exists() && create_dirs {
            let _ = create_dir_all(&path);
        }
        path
    }
    pub fn start(self) -> image::ImageResult<PathBuf> {
        let out_dir = self.get_output_dir(true);
        if self.format.can_write() && self.format != ImageFormat::Avif {
            let dyn_image = image::open(&self.file)?;
            dyn_image.save_with_format(
                format!(
                    "{}/{}.{}",
                    out_dir.to_str().ok_or(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Cannot the input file name"
                    ))?,
                    self.file.file_name().and_then(|name| name.to_str()).ok_or(
                        std::io::Error::new(
                            std::io::ErrorKind::NotFound,
                            "Cannot the input file name"
                        )
                    )?,
                    self.format.extensions_str()[0]
                ),
                self.format,
            )?;
            Ok(out_dir)
        } else {
            Ok(out_dir)
        }
    }
}
