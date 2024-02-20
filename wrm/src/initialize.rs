use crate::{FilesInTrash, FILES_IN_TRASH, TRASH};
use filey::{Error::FileyError, Filey, Result};

pub fn initialize() -> Result<()> {
    let mut trash = Filey::new(TRASH);

    trash.expand_user()?.absolutize()?;

    if !trash.exists() {
        trash.create_dir()?;
    }

    let mut files_in_trash = Filey::new(FILES_IN_TRASH);

    files_in_trash.expand_user()?.absolutize()?;

    if !files_in_trash.exists() {
        FilesInTrash::new(vec![])
            .write(files_in_trash)
            .map_err(|e| e.into())
            .map_err(FileyError)?;
    }

    Ok(())
}
