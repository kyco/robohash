mod hash;
mod image;
mod materials;

pub struct RoboHash {
    hash: String,
    sets: Vec<String>,
    hash_array_chunks: usize,
    selected_set_default_index: usize,
}

impl RoboHash {
    pub fn new(string: &str) -> anyhow::Result<Self> {
        let hash = hash::sha512_digest(string)?;
        let sets = materials::sets()?;
        let hash_array_chunks = 11;
        let selected_set_default_index = 1;
        Ok(Self {
            hash,
            sets,
            hash_array_chunks,
            selected_set_default_index,
        })
    }

    pub fn assemble_base64(&self) -> anyhow::Result<String> {
        let hash_array = hash::split_hash(&self.hash, self.hash_array_chunks)?;
        let robo_set = self.selected_set(&hash_array)?;
        let selected_files_in_set = self.select_files_in_set(&hash_array, &robo_set)?;
        let image = image::build_robo_hash_image(&selected_files_in_set)?;
        let image_string = image::to_base_64(&image)?;
        Ok(image_string)
    }

    fn selected_set(&self, hash_array: &Vec<i64>) -> anyhow::Result<String> {
        let set_index = self.default_set_index(hash_array);
        let selected_set = self.sets.get(set_index).unwrap().to_string();
        Ok(selected_set)
    }

    fn default_set_index(&self, hash_array: &Vec<i64>) -> usize {
        (hash_array[self.selected_set_default_index] % self.sets.len() as i64) as usize
    }

    fn select_files_in_set(&self, hash_array: &Vec<i64>, set: &str) -> anyhow::Result<Vec<String>> {
        let categories_in_set = materials::categories_in_set(set)?;
        let mut index = 4;
        let mut files = categories_in_set
            .iter()
            .flat_map(|category| {
                if let Ok(file) = materials::files_in_category(set, category) {
                    let set_index = hash_array[index] % file.len() as i64;
                    let selected_file = file.get(set_index as usize).unwrap().to_string();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_instance_of_robo_hash_returns_robo_hash_struct() {
        // arrange
        let initial_string = "initial_string";
        let expected_hash = "92ba5204aca5e21f60d40dda5b64e0e64e46028da5d33d2b577a0c80b6ed2843b46a458bbb0023d2634ecc7bccb2678e0b33f5ec0144fb124174325113396ef4";
        // act
        let robo_hash = RoboHash::new(initial_string);
        // assert
        assert_eq!(robo_hash.unwrap().hash, expected_hash)
    }

    #[test]
    fn new_instance_of_robo_hash_returns_robo_hash_struct_with_sets() {
        // arrange
        let initial_string = "initial_string";
        let sets = vec!["set1", "set2", "set3", "set4", "set5"];
        // act
        let robo_hash = RoboHash::new(initial_string);
        // assert
        assert_eq!(robo_hash.unwrap().sets, sets)
    }

    #[test]
    fn assemble_base64_returns_base64_encoded_image_of_robo_hash() {
        // arrange
        let initial_string = "initial_string";
        let robo_hash = RoboHash::new(initial_string).unwrap();
        // act
        let files = robo_hash.assemble_base64();
        // assert
        assert!(files.is_ok())
    }

    #[test]
    fn selected_set_returns_name_of_selected_set() {
        // arrange
        let initial_string = "initial_string";
        let hash_array = vec![
            10083058600650,
            6468747187213,
            15005379333732,
            15693853337043,
            4203522531528,
            785662886836,
            7302933098498,
            4202144124027,
            14066663350451,
            4354761377019,
            1254520726801,
            10083058600650,
            6468747187213,
            15005379333732,
            15693853337043,
            4203522531528,
            785662886836,
            7302933098498,
            4202144124027,
            14066663350451,
            4354761377019,
            1254520726801,
        ];
        let robo_hash = RoboHash::new(initial_string).unwrap();
        // act
        let set = robo_hash.selected_set(&hash_array);
        // assert
        assert!(set.is_ok());
        assert_eq!(set.unwrap(), "set4")
    }

    #[test]
    fn select_files_in_set_test() {
        // arrange
        let initial_string = "initial_string";
        let selected_set = "set4";
        let hash_array = vec![
            10083058600650,
            6468747187213,
            15005379333732,
            15693853337043,
            4203522531528,
            785662886836,
            7302933098498,
            4202144124027,
            14066663350451,
            4354761377019,
            1254520726801,
            10083058600650,
            6468747187213,
            15005379333732,
            15693853337043,
            4203522531528,
            785662886836,
            7302933098498,
            4202144124027,
            14066663350451,
            4354761377019,
            1254520726801,
        ];
        let robo_hash = RoboHash::new(initial_string).unwrap();

        // act
        let files = robo_hash.select_files_in_set(&hash_array, selected_set);
        // assert
        assert!(files.is_ok())
    }
}
