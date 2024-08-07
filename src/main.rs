use image::ImageFormat;
use image_benches::ConvertToParam;
use rayon::prelude::*;
use std::{
    fs::{create_dir_all, read_dir},
    path::Path,
};

fn main() -> anyhow::Result<()> {
    let input_dir = "./images";
    let output_dir = Path::new("./output/").to_path_buf();
    let all_format = ImageFormat::all().collect::<Vec<_>>();
    create_dir_all(&output_dir)?;
    read_dir(input_dir)?
        .flatten()
        .map(|entry| entry.path())
        .collect::<Vec<_>>()
        .into_par_iter()
        .for_each(move |file| {
            let output_dir = output_dir.clone();
            all_format.clone().into_par_iter().for_each(move |format| {
                let con = ConvertToParam {
                    file: file.clone(),
                    format,
                    output_dir: output_dir.clone(),
                };
                if let Err(e) = con.start() {
                    eprintln!("{e}");
                };
            });
        });
    Ok(())
}
