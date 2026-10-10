use parking_lot::RwLock;
use std::sync::Arc;
use crate::tui::PostReviewer;
use crate::{network::ServerList, tui::App};
use crate::posts::Post;

use std::{
    env::{temp_dir, var},
    fs::File,
    process::Command,
    path::PathBuf,
};

// https://docs.rs/capitalize/latest/capitalize/index.html
use capitalize::Capitalize;

use anyhow::Error;

// TODO: PublishingError
pub fn process(server_list_lock: Arc<RwLock<ServerList>>, title: Option<String>, path: Option<PathBuf>) -> Result<(), Error> {
    let server_list = server_list_lock.read();
    if !server_list.get_default()?.is_signed_in() {
        Err(Error::msg(format!("You are not signed in to {}!", server_list.get_default()?.url_string())))?
    }

    let title = match title {
        Some(s) => s,
        None => format_title(
            String::from(
                path.as_ref().expect("The path should be Some, since title is None")
                .file_prefix().ok_or(Error::msg("The file had a weird name; I couldn't get a title from it"))?
                .to_str().ok_or(Error::msg("The file's name had some Unicode issues; I couldn't get a title from it"))?
            )
        )
    };
    
    let content = match &path {
        Some(path) => std::fs::read_to_string(&path)
        .or(Err(Error::msg("I couldn't read the path you gave me")))?,
        None => get_editor_input(None)?,
    };

    let mut post = Post {title: title.clone(), content: content};

    loop {
        match App::default().run(&mut PostReviewer::new(post.clone()))? {
            None => {return Ok(())},
            Some(false) => {
                let content = get_editor_input(path.clone().or(Some(get_temp_post_path()?)))?;
                post.content = content;
            }
            Some(true) => {break}
        }
    }

    // TODO: right here a no-server error needs to be printed somehow...?
    let message = server_list.publish(server_list.get_default()?, post)?;

    println!("{message}");
    Ok(())
}

fn format_title(title: String) -> String {
    return title.split(|x: char| x.is_whitespace() || x == '-' || x == '_')
    // I don't have to capitalize myself, thanks to this crate...
    // https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
    .map(|x: &str| x.capitalize())
    .collect::<Vec<String>>()
    .join(" ")
}

fn get_temp_post_path() -> Result<PathBuf, Error> {
    let mut file_path = temp_dir();
    file_path.push("terse-post");
    return Ok(file_path);
}

// https://stackoverflow.com/questions/56011927/how-do-i-use-rust-to-open-the-users-default-editor-and-get-the-edited-content
/// If path is null, an empty temp file is created.
fn get_editor_input(path: Option<PathBuf>) -> Result<String, Error> {
    let editor = match var("EDITOR") {
        Ok(v) => v,
        Err(_) => String::from("vim")
    };
    
    let file_path = match path {
        Some(p) => p,
        None => {
            let p = get_temp_post_path()?;
            File::create(&p).or(Err(Error::msg("I couldn't create a temporary file to store your post in")))?;
            p
        }
    };

    Command::new(&editor)
        .arg(&file_path)
        .status()
        .or(Err(Error::msg("I had a problem opening {&file_path} (using {editor})")))?;

    return Ok(std::fs::read_to_string(&file_path)?)
}
