use crate::{
    exists_or_else, utils::*, Error::NotFound, File, FilesInTrash, Options, Result, FILES_IN_TRASH,
};
use colored::Colorize;
use filey::Filey;

pub fn remove(
    paths: &Vec<String>,
    files_in_trash: &mut FilesInTrash,
    options: &Options,
) -> Result<()> {
    for path in paths {
        exists_or_else!(path);

        if !options.noninteractive() {
            let message = format!("{} '{}' to trash?", "Move".red().bold(), path);

            if !confirm(message)? {
                eprintln!("Canceled");

                continue;
            }
        }

        let file = File::new(path)?;

        if let Err(e) = rename(file.from(), file.path()) {
            eprintln(e);

            continue;
        }

        files_in_trash
            .add(file)
            .write(absolutize(FILES_IN_TRASH)?)?;

        if !options.quiet() {
            eprintln!("{} '{}' to trash", "Moved".green().bold(), path);
        }
    }

    Ok(())
}
