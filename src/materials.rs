use crate::error::Error;
use std::env;
use std::path::{Path, PathBuf};

pub(crate) fn categories_in_set(sets_root: &str, set: &str) -> Result<Vec<String>, Error> {
    let current_dir = get_current_working_dir()?;
    let sets_dir = current_dir.join(Path::new(sets_root)).join(set);
    let sets = directories_in_path(&sets_dir)?;
    Ok(sets)
}

pub(crate) fn files_in_category(
    sets_root: &str,
    set: &str,
    category: &str,
) -> Result<Vec<String>, Error> {
    let current_dir = get_current_working_dir()?;
    let sets_dir = path_builder(sets_root, &current_dir, set, category);
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

fn path_builder(sets_root: &str, current_dir: &PathBuf, set: &str, category: &str) -> PathBuf {
    current_dir
        .join(Path::new(sets_root))
        .join(set)
        .join(category)
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
