use crate::error::Error;
use std::env;
use std::path::{Path, PathBuf};

pub(crate) const SETS_PATH: &str = "sets";

pub(crate) fn sets() -> Result<Vec<String>, Error> {
    let current_dir = get_current_working_dir()?;
    let sets_dir = current_dir.join(Path::new(SETS_PATH));
    let sets = directories_in_path(&sets_dir)?;
    Ok(sets)
}

pub(crate) fn categories_in_set(set: &str) -> Result<Vec<String>, Error> {
    let current_dir = get_current_working_dir()?;
    let sets_dir = current_dir.join(Path::new(SETS_PATH)).join(set);
    let sets = directories_in_path(&sets_dir)?;
    Ok(sets)
}

pub(crate) fn files_in_category(set: &str, category: &str) -> Result<Vec<String>, Error> {
    let current_dir = get_current_working_dir()?;
    let sets_dir = path_builder(set, category, &current_dir);
    let sets = directories_in_path(&sets_dir)?
        .iter()
        .flat_map(|dir| {
            if let Some(path) = sets_dir.join(dir).as_path().to_str() {
                Some(String::from(path))
            } else {
                println!("cannot create directory {sets_dir:#?}/{dir:#?}");
                None
            }
        })
        .collect::<Vec<String>>();
    Ok(sets)
}

fn path_builder(set: &str, category: &str, current_dir: &PathBuf) -> PathBuf {
    current_dir
        .join(Path::new(SETS_PATH))
        .join(set)
        .join(category)
}

pub(crate) fn colours() -> Result<Vec<String>, Error> {
    let current_dir = get_current_working_dir()?;
    let sets_dir = current_dir.join(Path::new(SETS_PATH)).join("set1");
    let sets = directories_in_path(&sets_dir)?;
    Ok(sets)
}

pub(crate) fn backgrounds() -> Result<Vec<String>, Error> {
    let current_dir = get_current_working_dir()?;
    let backgrounds_dir = current_dir.join(Path::new("backgrounds"));
    let backgrounds = directories_in_path(&backgrounds_dir)?;
    Ok(backgrounds)
}

fn directories_in_path(path: &PathBuf) -> Result<Vec<String>, Error> {
    let mut directories = path
        .read_dir()?
        .into_iter()
        .filter_map(|path| match path {
            Ok(path) => match path.file_name().into_string() {
                Ok(set) => Some(set),
                Err(e) => {
                    println!("{e:#?}");
                    None
                }
            },
            Err(_) => None,
        })
        .collect::<Vec<String>>();
    directories.sort();
    Ok(directories)
}

fn get_current_working_dir() -> Result<PathBuf, Error> {
    Ok(env::current_dir()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_reads_all_directories_in_sets_directory() {
        // arrange
        // act
        let sets = sets();
        // assert
        assert!(sets.is_ok());
        assert_eq!(sets.unwrap().len(), 5)
    }

    #[test]
    fn backgrounds_reads_all_directories_in_backgrounds_directory() {
        // arrange
        // act
        let backgrounds = backgrounds();
        // assert
        assert!(backgrounds.is_ok());
        assert_eq!(backgrounds.unwrap().len(), 2)
    }

    #[test]
    fn colours_reads_all_directories_in_colours_directory() {
        // arrange
        // act
        let colours = colours();
        // assert
        assert!(colours.is_ok());
        assert_eq!(colours.unwrap().len(), 10)
    }
}
