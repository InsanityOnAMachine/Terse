use parking_lot::RwLock;
use std::sync::Arc;
use crate::network::ServerList;
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


pub fn process(server_list_lock: Arc<RwLock<ServerList>>, title: Option<String>, path: Option<PathBuf>) -> Result<(), Error> {
    let server_list = server_list_lock.read();

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
    
    let content = match path {
        Some(path) => std::fs::read_to_string(&path)
        .or(Err(Error::msg("I couldn't read the path you gave me")))?,
        None => get_editor_input()?,
    };

    if cfg!(debug_assertions) {
        println!("Title: {title}");
        println!("Content: \n{content}")
    }

    let post = Post {title: title.clone(), content: content};

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

// https://stackoverflow.com/questions/56011927/how-do-i-use-rust-to-open-the-users-default-editor-and-get-the-edited-content
fn get_editor_input() -> Result<String, Error> {
    let editor = match var("EDITOR") {
        Ok(v) => v,
        Err(_) => String::from("vim")
    };
    
    let mut file_path = temp_dir();
    file_path.push("terse-post");
    File::create(&file_path).or(Err(Error::msg("I couldn't create a temporary file to store your post in")))?;

    Command::new(&editor)
        .arg(&file_path)
        .status()
        .or(Err(Error::msg("I had a problem opening {&file_path} (using {editor})")))?;

    return Ok(std::fs::read_to_string(&file_path)?)
}