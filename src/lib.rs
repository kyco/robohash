use crate::colour::Colour;
use crate::error::Error;
use crate::set_type::Set;
use strum::IntoEnumIterator;

pub mod colour;
pub mod error;
mod hash;
mod image;
mod materials;
pub mod set_type;

pub struct RoboHashBuilder {
    text: String,
    set: Set,
    set_root: String,
    colour: Colour,
    image_size: ImageSize,
}

impl RoboHashBuilder {
    pub fn new(text: &str) -> Self {
        let set = Set::Default;
        let colour = Colour::Any;
        let set_root = String::from("./sets");
        let image_size = ImageSize::default();
        Self {
            text: String::from(text),
            set,
            set_root,
            colour,
            image_size,
        }
    }

    pub fn with_set(mut self, set: Set) -> RoboHashBuilder {
        self.set = set;
        self
    }

    pub fn with_set_location(mut self, set_location: &str) -> RoboHashBuilder {
        self.set_root = String::from(set_location);
        self
    }

    pub fn with_colour(mut self, colour: Colour) -> RoboHashBuilder {
        self.colour = colour;
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> RoboHashBuilder {
        self.image_size = ImageSize { width, height };
        self
    }

    pub fn build(&self) -> Result<RoboHash, Error> {
        let hash_array_chunks = 11;
        let hash = hash::sha512_digest(&self.text)?;
        let hash_array = hash::split_hash(&hash, hash_array_chunks)?;

        let colour = colour_selection(&hash_array, &self.colour, &self.set);
        let set = match self.set {
            Set::Default | Set::Set1 => {
                format!("{}/{}", &self.set.as_str(), colour.as_str())
            }
            _ => String::from(self.set.as_str()),
        };
        let sets_root = self.set_root.to_owned();

        Ok(RoboHash {
            image_size: self.image_size,
            hash_array,
            set,
            sets_root,
        })
    }
}

#[derive(Debug)]
pub struct RoboHash {
    image_size: ImageSize,
    hash_array: Vec<i64>,
    set: String,
    sets_root: String,
}

#[derive(Debug, Clone, Copy)]
struct ImageSize {
    width: u32,
    height: u32,
}

impl ImageSize {
    pub(crate) fn default() -> Self {
        Self {
            width: 1024,
            height: 1024,
        }
    }
}

impl RoboHash {
    pub fn assemble_base64(&self) -> Result<String, Error> {
        if self.is_missing_required_data() {
            return Err(Error::RoboHashMissingRequiredData);
        }

        let set = files_in_set(&self.hash_array, &self.sets_root, &self.set)?;
        let image =
            image::build_robo_hash_image(&set, self.image_size.width, self.image_size.height)?;
        let base64 = image::to_base_64(&image)?;
        Ok(base64)
    }

    fn is_missing_required_data(&self) -> bool {
        self.hash_array.is_empty() || self.set.is_empty() || self.sets_root.is_empty()
    }
}

fn files_in_set(hash_array: &Vec<i64>, sets_root: &str, set: &str) -> Result<Vec<String>, Error> {
    let categories_in_set = materials::categories_in_set(sets_root, set)?;
    let mut index = 4;
    let mut files = categories_in_set
        .iter()
        .flat_map(
            |category| match materials::files_in_category(sets_root, set, category) {
                Ok(file) => {
                    let set_index = (hash_array[index] % file.len() as i64) as usize;
                    if let Some(selected_file) = file.get(set_index) {
                        index = index + 1;
                        Some(String::from(selected_file))
                    } else {
                        println!("failed to fetch index {set_index:#?} from {file:#?}");
                        None
                    }
                }
                Err(e) => {
                    println!("{e:#?}");
                    None
                }
            },
        )
        .collect::<Vec<String>>();
    files.sort_by(|a, b| {
        a.split("#").collect::<Vec<_>>()[1].cmp(b.split("#").collect::<Vec<_>>()[1])
    });
    Ok(files)
}

fn colour_selection(hash_array: &Vec<i64>, colour: &Colour, set: &Set) -> Colour {
    let is_default_set_with_any_colour =
        (set == &Set::Set1 || set == &Set::Default) && colour == &Colour::Any;
    let is_not_set_1_and_not_any_colour =
        (set != &Set::Set1 && set != &Set::Default) && set != &Set::Default;

    if is_default_set_with_any_colour || is_not_set_1_and_not_any_colour {
        random_colour(hash_array)
    } else {
        colour.clone()
    }
}

fn random_colour(hash_array: &Vec<i64>) -> Colour {
    let mut available_colours: Vec<Colour> = Colour::iter().collect();
    available_colours.retain(|colour| colour != &Colour::Any);
    let selected_index = (hash_array[0] % available_colours.len() as i64) as usize;
    available_colours[selected_index].clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image::tests::load_base64_string_image_resources;

    #[test]
    fn test_that_robo_hash_builder_returns_a_builder() {
        // arrange
        let text = "text";
        // act
        let robo_hash_builder = RoboHashBuilder::new(text);
        // assert
        assert_eq!(robo_hash_builder.text, text)
    }

    #[test]
    fn test_that_robo_hash_builder_returns_a_builder_with_default_set() {
        // arrange
        let text = "text";
        let expected_set = Set::Default;
        // act
        let robo_hash_builder = RoboHashBuilder::new(text);
        // assert
        assert_eq!(robo_hash_builder.set, expected_set)
    }

    #[test]
    fn test_that_robo_hash_builder_returns_a_builder_with_colour_set_to_any() {
        // arrange
        let text = "text";
        let expected_colour = Colour::Any;
        // act
        let robo_hash_builder = RoboHashBuilder::new(text);
        // assert
        assert_eq!(robo_hash_builder.colour, expected_colour)
    }

    #[test]
    fn test_that_robo_hash_builder_with_set_changes_the_set() {
        // arrange
        let text = "text";
        let set = Set::Set3;
        let expected_set = Set::Set3;
        // act
        let robo_hash_builder = RoboHashBuilder::new(text).with_set(set);
        // assert
        assert_eq!(robo_hash_builder.set, expected_set)
    }

    #[test]
    fn test_that_robo_hash_builder_with_colour_changes_sets_colour() {
        // arrange
        let text = "text";
        let colour = Colour::Blue;
        let expected_colour = Colour::Blue;
        // act
        let robo_hash_builder = RoboHashBuilder::new(text).with_colour(colour);
        // assert
        assert_eq!(robo_hash_builder.colour, expected_colour)
    }

    #[test]
    fn test_that_robo_hash_builder_with_set_root_changes_sets_new_set_root() {
        // arrange
        let text = "text";
        let set_root = "new_set_root";
        let expected_set_root = "new_set_root";
        // act
        let robo_hash_builder = RoboHashBuilder::new(text).with_set_location(set_root);
        // assert
        assert_eq!(robo_hash_builder.set_root, expected_set_root)
    }

    #[test]
    fn test_that_robo_hash_builder_build_returns_a_robo_hash_struct() {
        // arrange
        let text = "text";
        let expected_hash_array = vec![
            16145521472556,
            12696294247384,
            5154811788184,
            10555455865428,
            2642153577670,
            16342997499342,
            10550500569788,
            8328031981449,
            14915230302908,
            14678679777589,
            12705535333312,
            16145521472556,
            12696294247384,
            5154811788184,
            10555455865428,
            2642153577670,
            16342997499342,
            10550500569788,
            8328031981449,
            14915230302908,
            14678679777589,
            12705535333312,
        ];
        // act
        let robo_hash = RoboHashBuilder::new(text).build();
        // assert
        assert!(robo_hash.is_ok());
        assert_eq!(robo_hash.unwrap().hash_array, expected_hash_array)
    }

    #[test]
    fn test_robo_hash_assemble_base64_returns_missing_data_error_when_robo_hash_does_not_contain_hash_array(
    ) {
        // arrange
        let image_size = ImageSize {
            width: 1024,
            height: 1024,
        };
        let robo_hash = RoboHash {
            image_size,
            hash_array: vec![],
            set: String::from("set1"),
            sets_root: String::from("set_root"),
        };
        // act
        let image = robo_hash.assemble_base64();
        // assert
        assert!(image.is_err());
        assert_eq!(
            image.err().unwrap().to_string(),
            Error::RoboHashMissingRequiredData.to_string()
        )
    }

    #[test]
    fn test_robo_hash_assemble_base64_returns_missing_data_error_when_set_does_not_contain_any_data(
    ) {
        // arrange
        let image_size = ImageSize {
            width: 1024,
            height: 1024,
        };
        let robo_hash = RoboHash {
            image_size,
            hash_array: vec![1, 2],
            set: String::from(""),
            sets_root: String::from("set_root"),
        };
        // act
        let image = robo_hash.assemble_base64();
        // assert
        assert!(image.is_err());
        assert_eq!(
            image.err().unwrap().to_string(),
            Error::RoboHashMissingRequiredData.to_string()
        )
    }

    #[test]
    fn test_robo_hash_assemble_base64_returns_missing_data_error_when_sets_root_does_not_contain_any_data(
    ) {
        // arrange
        let image_size = ImageSize {
            width: 1024,
            height: 1024,
        };
        let robo_hash = RoboHash {
            image_size,
            hash_array: vec![1, 2],
            set: String::from("set1"),
            sets_root: String::from(""),
        };
        // act
        let image = robo_hash.assemble_base64();
        // assert
        assert!(image.is_err());
        assert_eq!(
            image.err().unwrap().to_string(),
            Error::RoboHashMissingRequiredData.to_string()
        )
    }

    #[test]
    #[ignore]
    fn test() {
        // setup
        let initial_string = "test";
        let sets: Vec<Set> = Set::iter().collect();
        let colours: Vec<Colour> = Colour::iter().collect();

        for set in sets {
            for colour in colours.clone() {
                // arrange
                let set = set.clone();
                let test_resource = format!("{initial_string:#?}_{set:#?}_{colour:#?}");
                let expected_robo_hash = load_base64_string_image_resources(&test_resource);

                // act
                let robo_hash = RoboHashBuilder::new(initial_string)
                    .with_set(set)
                    .with_colour(colour)
                    .build()
                    .unwrap();
                let constructed_robo_hash = robo_hash.assemble_base64().unwrap();

                // assert
                assert_eq!(constructed_robo_hash, expected_robo_hash);
            }
        }
    }
}
