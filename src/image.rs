use crate::error::Error;
use image::{DynamicImage, RgbaImage};
use std::io::Cursor;

pub(crate) fn build_robo_hash_image(robo_parts: &Vec<String>) -> Result<RgbaImage, Error> {
    let mut base_image = image::ImageBuffer::new(1024, 1024);
    robo_parts
        .iter()
        .try_for_each(|image_path| -> Result<(), Error> {
            let mut image = try_open_image(image_path)?;
            image::imageops::overlay(&mut base_image, &mut image, 0, 0);
            Ok(())
        })?;
    Ok(base_image)
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
    use super::*;
    use std::fs::File;
    use std::io::Read;

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
        let robo_hash = build_robo_hash_image(&robo_parts);
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
        let robo_hash = build_robo_hash_image(&robo_parts).unwrap();
        // act
        let base64_string = to_base_64(&robo_hash);
        // assert
        assert!(base64_string.is_ok());
        assert_eq!(base64_string.unwrap(), expected_base64)
    }

    pub(crate) fn load_base64_string_image_resources(filename: &str) -> String {
        let mut file_contents = String::new();
        let mut file = File::open(&format!("./test_resources/{}.txt", filename)).unwrap();
        file.read_to_string(&mut file_contents).unwrap();
        file_contents
    }
}
