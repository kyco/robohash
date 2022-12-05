use std::env;
use std::path::{Path, PathBuf};

pub(crate) fn sets() -> anyhow::Result<Vec<String>> {
    let current_dir = get_current_working_dir()?;
    let sets_dir = current_dir.join(Path::new("sets"));
    let sets = dirs_in_path(&sets_dir)?;
    Ok(sets)
}

pub(crate) fn categories_in_set(set: &str) -> anyhow::Result<Vec<String>> {
    let current_dir = get_current_working_dir()?;
    let sets_dir = current_dir.join(Path::new("sets")).join(set);
    let sets = dirs_in_path(&sets_dir)?;
    Ok(sets)
}

pub(crate) fn files_in_category(set: &str, category: &str) -> anyhow::Result<Vec<String>> {
    let current_dir = get_current_working_dir()?;
    let sets_dir = current_dir.join(Path::new("sets")).join(set).join(category);
    let sets = dirs_in_path(&sets_dir)?
        .iter()
        .map(|dir| String::from(sets_dir.join(dir).as_path().to_str().unwrap()))
        .collect::<Vec<String>>();
    Ok(sets)
}

pub(crate) fn colours() -> anyhow::Result<Vec<String>> {
    let current_dir = get_current_working_dir()?;
    let sets_dir = current_dir.join(Path::new("sets")).join("set1");
    let sets = dirs_in_path(&sets_dir)?;
    Ok(sets)
}

pub(crate) fn backgrounds() -> anyhow::Result<Vec<String>> {
    let current_dir = get_current_working_dir()?;
    let backgrounds_dir = current_dir.join(Path::new("backgrounds"));
    let backgrounds = dirs_in_path(&backgrounds_dir)?;
    Ok(backgrounds)
}

fn dirs_in_path(path: &PathBuf) -> anyhow::Result<Vec<String>> {
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

fn get_current_working_dir() -> anyhow::Result<PathBuf> {
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
