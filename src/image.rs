use std::io::Cursor;

use image::{imageops, DynamicImage, ImageBuffer, Rgba, RgbaImage};

use crate::error::Error;

pub(crate) fn build_robo_hash_image(
    robo_parts: &Vec<String>,
    background: &Option<String>,
    width: u32,
    height: u32,
) -> Result<RgbaImage, Error> {
    let mut base_image = image::ImageBuffer::new(width, height);
    if let Some(background) = background {
        append_to_image(&mut base_image, background, width, height)?;
    }
    robo_parts
        .iter()
        .try_for_each(|image_path| -> Result<(), Error> {
            append_to_image(&mut base_image, image_path, width, height)?;
            Ok(())
        })?;
    Ok(base_image)
}

fn append_to_image(
    base_image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    image_path: &String,
    width: u32,
    height: u32,
) -> Result<(), Error> {
    let image = try_open_image(image_path)?;
    let mut image = imageops::resize(&image, width, height, imageops::FilterType::Lanczos3);
    imageops::overlay(base_image, &mut image, 0, 0);
    Ok(())
}

fn try_open_image(image_path: &String) -> Result<DynamicImage, Error> {
    match image::open(image_path) {
        Ok(image) => Ok(image),
        Err(e) => Err(Error::ImageOpenFailed(format!("{e:#?}"))),
    }
}

pub(crate) fn to_base_64(image: &RgbaImage) -> Result<String, Error> {
    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
    Ok(base64::encode(&bytes))
}

#[cfg(test)]
pub(crate) mod tests {
    use std::fs::File;
    use std::io::Read;

    use super::*;

    #[test]
    fn build_robo_hash_image_returns_built_image_of_parts() {
        // arrange
        let robo_parts = vec![
            String::from("./sets/set4/000#00body/003#body3.png"),
            String::from("./sets/set4/001#01fur/006#fur6.png"),
            String::from("./sets/set4/002#02eyes/008#eyes8.png"),
            String::from("./sets/set4/003#03mouth/007#mouth7.png"),
            String::from("./sets/set4/004#04accessories/003#accessory3.png"),
        ];
        // act
        let robo_hash = build_robo_hash_image(&robo_parts, &None, 512, 512);
        // assert
        assert!(robo_hash.is_ok())
    }

    #[test]
    fn to_base64_converts_image_to_base64_string() {
        // arrange
        let robo_parts = vec![
            String::from("./sets/set4/000#00body/003#body3.png"),
            String::from("./sets/set4/001#01fur/006#fur6.png"),
            String::from("./sets/set4/002#02eyes/008#eyes8.png"),
            String::from("./sets/set4/003#03mouth/007#mouth7.png"),
            String::from("./sets/set4/004#04accessories/003#accessory3.png"),
        ];
        let expected_base64 = load_base64_string_image_resources("image");
        let robo_hash = build_robo_hash_image(&robo_parts, &None, 512, 512).unwrap();
        // act
        let base64_string = to_base_64(&robo_hash);
        // assert
        assert!(base64_string.is_ok());
        assert_eq!(base64_string.unwrap(), expected_base64)
    }

    pub(crate) fn load_base64_string_image_resources(filename: &str) -> String {
        let mut file_contents = String::new();
        let file_location = format!("./test_resources/{}.txt", filename);
        let mut file = File::open(&file_location).unwrap();
        file.read_to_string(&mut file_contents).unwrap();
        file_contents
    }
}
