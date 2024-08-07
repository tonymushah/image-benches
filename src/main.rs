use std::fs::{create_dir_all, read_dir};

fn main() -> anyhow::Result<()> {
    let input_dir = "./images";
    let output_dir = "./output/jpeg";
    create_dir_all(output_dir)?;
    for img_path in read_dir(input_dir)?.flatten().map(|entry| entry.path()) {
        println!("filename = {}", img_path.to_str().unwrap_or_default());
        let image_ = image::open(&img_path)?;
        image_.save_with_format(
            format!(
                "{output_dir}/{}.jpeg",
                img_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .ok_or(anyhow::Error::msg("Cannot get the filename"))?
            ),
            image::ImageFormat::Jpeg,
        )?;
    }
    Ok(())
}
