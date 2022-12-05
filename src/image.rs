use image::RgbaImage;
use std::io::Cursor;

pub(crate) fn build_robo_hash_image(robo_parts: &Vec<String>) -> anyhow::Result<RgbaImage> {
    let mut base_image: RgbaImage = image::ImageBuffer::new(1024, 1024);
    robo_parts.iter().for_each(|image_path| {
        let mut image: RgbaImage = image::open(image_path).unwrap().to_rgba8();
        image::imageops::overlay(&mut base_image, &mut image, 0, 0);
    });
    Ok(base_image)
}

pub(crate) fn to_base_64(image: &RgbaImage) -> anyhow::Result<String> {
    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
    Ok(base64::encode(&bytes))
}

#[cfg(test)]
mod tests {
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
        let robo_hash = build_robo_hash_image(&robo_parts);
        // assert
        assert!(robo_hash.is_ok())
    }
}
