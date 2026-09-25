use std::fs::File;
use std::path::PathBuf;
use directories::ProjectDirs;

pub fn get_config_dir() -> Result<PathBuf, DataStorageError>{
    // We store to a different data file on debug builds vs. release builds
    // because the official data file is a pain to access...
    // https://stackoverflow.com/questions/39204908/how-to-check-release-debug-builds-using-cfg-in-rust
    Ok(
        if cfg!(not(debug_assertions)) {
            // https://stackoverflow.com/questions/37890405/is-there-a-way-to-simplify-converting-an-option-into-a-result-without-a-macro
            // https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.from
            let dirs = ProjectDirs::from("", "InsanityOnAMachine", "Terse")
            .ok_or(DataStorageError::BadFilesystemConfig)?;
            dirs.data_dir().to_path_buf()
        } else {
            PathBuf::from("./data")
        }
    )
}

pub fn ensure_config_file(file_path: PathBuf) -> Result<PathBuf, DataStorageError> {
    if !file_path.exists() {

        if let Some(folder_path) = file_path.parent() {
            std::fs::create_dir_all(folder_path)
            .or(Err(DataStorageError::CouldntCreateFile))?;
        }

        File::create(&file_path)
        .or(Err(DataStorageError::CouldntCreateFile))?;
    }

    Ok(file_path)
}

#[derive(thiserror::Error, Debug)]
pub enum DataStorageError {
    #[error("I couldn't decide where to look for the storage file, I couldn't find a $HOME base")]
    BadFilesystemConfig,
    #[error("I couldn't find the storage file, and I couldn't create a new file at the path where it should be")]
    CouldntCreateFile,
    #[error("I tried to read the storage file, but it contained bad data")]
    BadDataInFile,
    #[error("I couldn't successfully turn the data into JSON to save it in the storage file.\nThis shouldn't ever happen, lucky you!")]
    CantSerializeSelf,
    #[error("I couldn't read from the storage file")]
    CantReadFromFile,
    #[error("I couldn't save to the storage file")]
    CantWriteToFile,
}
