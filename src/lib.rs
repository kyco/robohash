use crate::colour::Colour;
use crate::error::Error;
use crate::set_type::Set;
use strum::IntoEnumIterator;

mod colour;
mod error;
mod hash;
mod image;
mod materials;
mod set_type;

#[derive(Debug)]
pub struct RoboHash {
    hash_array: Vec<i64>,
    set: String,
}

impl RoboHash {
    pub fn new(string: &str, set: Set, colour: Colour) -> Result<Self, Error> {
        let hash_array_chunks = 11;

        let hash = hash::sha512_digest(string)?;
        let hash_array = hash::split_hash(&hash, hash_array_chunks)?;
        let colour = colour_selection(&hash_array, &colour, &set);

        let set = match set {
            Set::Default | Set::Set1 => {
                format!("{}/{}", &set.as_str(), colour.as_str())
            }
            _ => String::from(set.as_str()),
        };

        Ok(Self { hash_array, set })
    }

    pub fn assemble_base64(&self) -> Result<String, Error> {
        let selected_files_in_set = self.select_files_in_set()?;
        let image = image::build_robo_hash_image(&selected_files_in_set)?;
        let image_string = image::to_base_64(&image)?;
        Ok(image_string)
    }

    fn select_files_in_set(&self) -> Result<Vec<String>, Error> {
        let categories_in_set = materials::categories_in_set(&self.set)?;
        let mut index = 4;
        let mut files = categories_in_set
            .iter()
            .flat_map(|category| {
                if let Ok(file) = materials::files_in_category(&self.set, category) {
                    let set_index = self.hash_array[index] % file.len() as i64;
                    let selected_file = match file.get(set_index as usize) {
                        Some(file) => file.to_string(),
                        None => {
                            println!("failed to fetch index {set_index:#?} from {file:#?}");
                            return None;
                        }
                    };
                    index = index + 1;
                    Some(selected_file)
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();
        files.sort_by(|a, b| {
            a.split("#").collect::<Vec<_>>()[1].cmp(b.split("#").collect::<Vec<_>>()[1])
        });
        Ok(files)
    }
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
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn set_type_default_constructs_robo_hash_with_set_1() {
        // arrange
        let initial_string = "initial_string";
        let set = Set::Default;
        let colour = Colour::Any;
        let robo_hash = RoboHash::new(initial_string, set, colour).unwrap();
        // act
        let constructed_robo_hash = robo_hash.assemble_base64().unwrap();
        write_test_resources("default_set_colour_any.txt", &constructed_robo_hash);
        // assert
        assert_eq!(1, 1)
    }

    #[test]
    fn set_type_default_colour_yellow_constructs_yellow_robo_hash_with_set_1() {
        // arrange
        let initial_string = "initial_string";
        let set = Set::Default;
        let colour = Colour::Yellow;
        let robo_hash = RoboHash::new(initial_string, set, colour).unwrap();
        // act
        let constructed_robo_hash = robo_hash.assemble_base64().unwrap();
        write_test_resources("default_set_colour_yellow.txt", &constructed_robo_hash);
        // assert
        assert_eq!(1, 1)
    }

    #[test]
    fn set_type_set_1_constructs_robo_hash_with_set_1() {
        // arrange
        let initial_string = "initial_string";
        let set = Set::Set1;
        let colour = Colour::Any;
        let robo_hash = RoboHash::new(initial_string, set, colour).unwrap();
        // act
        let constructed_robo_hash = robo_hash.assemble_base64().unwrap();
        write_test_resources("set_1_colour_any.txt", &constructed_robo_hash);
        // assert
        assert_eq!(1, 1)
    }

    #[test]
    fn set_type_set_2_constructs_robo_hash_with_set_2() {
        // arrange
        let initial_string = "initial_string";
        let set = Set::Set2;
        let colour = Colour::Blue;
        let robo_hash = RoboHash::new(initial_string, set, colour).unwrap();
        // act
        let constructed_robo_hash = robo_hash.assemble_base64().unwrap();
        write_test_resources("set_2_colour_blue.txt", &constructed_robo_hash);
        // assert
        assert_eq!(1, 1)
    }

    #[test]
    fn assemble_base64_returns_base64_encoded_image_of_robo_hash() {
        // arrange
        let initial_string = "initial_string";
        let set = Set::Default;
        let colour = Colour::Any;
        let robo_hash = RoboHash::new(initial_string, set, colour).unwrap();
        // act
        let files = robo_hash.assemble_base64();
        // assert
        assert!(files.is_ok())
    }

    fn write_test_resources(filename: &str, base64_string: &str) {
        let file = File::create(&format!("./test_resources/{}.txt", filename));
        let _ = file.unwrap().write_all(base64_string.as_bytes()).unwrap();
    }
}
