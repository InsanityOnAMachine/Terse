use parking_lot::RwLock;
use std::sync::Arc;
use clap::{Parser, Subcommand};
use anyhow::Error;

use url::Url;
use std::path::PathBuf;

use crate::tui::{App, SearchMenu};
use crate::network::{LoginInfo, ServerList};
use crate::data::{self, DataStorageError};

pub mod server_subcommand;
use server_subcommand::*;

pub mod pub_command;

#[cfg(debug_assertions)]
pub mod dev_subcommand;
#[cfg(debug_assertions)]
use dev_subcommand::*;

// https://docs.rs/clap/latest/clap/_derive/
// NOTE: I could move stats into the server and so go --server stats?...

// This helped me get a Vec<String> into an enum:
// https://github.com/clap-rs/clap/blob/master/examples/tutorial_derive/03_04_subcommands.rs

#[derive(Parser)]
#[command(disable_help_flag = true, override_help = include_str!("docs/main.txt"))]
pub enum Cli {
    #[command(name = "--search", override_help = include_str!("docs/search.txt"))]
    Search {query: Vec<String>},

    #[command(name = "--stats",  override_help = include_str!("docs/stats.txt"))]
    Stats,

    #[command(name = "--pub",    override_help = include_str!("docs/pub.txt"))]
    #[group(required=true)]
    Pub {#[arg(short)] title: Option<String>, #[arg(short)] path: Option<PathBuf>},

    #[command(subcommand, name = "--server", override_help = include_str!("docs/server/main.txt"))]
    Server(ServerSubcommand),

    #[command(name = "--whoami", override_help = include_str!("docs/whoami.txt"))]
    Whoami,
    
    // eeyou do not GET access to this pro-prietary subcommando!!!
    #[cfg(debug_assertions)]
    #[command(subcommand, name = "--dev", about = "Special developer commands to do things faster")]
    Dev(DevSubcommand),
}

impl Cli {
    pub fn from_args() -> Self {

        // if the first argument isn't a flag (doesn't start with -) and isn't 'help',
        // that's a shortcut for searching; so we insert a --search command
        // and parse from it, for your convenience!

        if let Some(command) = std::env::args().nth(1) {

            // Why does pythonexamples.org have rust tutorials?!
            // https://pythonexamples.org/rust/how-to-get-first-n-characters-in-string
            // https://www.dotnetperls.com/starts-with-rust

            if !(command.starts_with("-") || command == "help"){
                // search shortcut! Stick a --search in there and parse it!
                let mut search_insert = std::env::args().into_iter().collect::<Vec<String>>();
                search_insert.insert(1, "--search".to_string());

                return Self::parse_from(search_insert);
            }
        }

        return Self::parse()
    }


    pub fn get_server_list() -> Result<(PathBuf, Arc<RwLock<ServerList>>), DataStorageError> {
        // https://users.rust-lang.org/t/tuple-of-results-into-a-result-of-tuples/120191/2
        let server_file = data::ensure_config_file(data::get_config_dir()?.join("servers.json"))?;
        Ok((server_file.clone(), Arc::new(RwLock::new(ServerList::from_config_file(server_file.clone())?))))
    }

    pub fn process(self) -> Result<(), Error> {
        // Having this here does mean that some commands that don't need
        // disk access at all could fail, but it is WORTH IT.
        // Besides, if you can't read the config file that's enough of an 
        // issue to stop doing anything.

        let (server_file, server_list) = match Self::get_server_list() {
            Ok(s) => s,
            Err(e) => {
                return Err(Error::msg(format!("I had a problem reading from disk: {e}")))
            }
        };

        match self {
            Cli::Search {query}    => Self::process_query(query, server_list.clone()),
            Cli::Stats             => Self::process_stats(server_list.clone()),
            Cli::Pub {title, path} => pub_command::process(server_list.clone(), title, path),
            Cli::Server(command)   => command.process(server_list.clone()),
            Cli::Whoami            => Self::process_whoami(server_list.clone()),

            #[cfg(debug_assertions)]
            Cli::Dev(command)      => command.process(),
        }?;

        if let Err(e) = server_list.read().store(server_file) {
            return Err(Error::msg(format!("I got an error when trying to write to disk: \n{e}")))
        }

        Ok(())
    }

    fn process_query(words: Vec<String>, server_list_lock: Arc<RwLock<ServerList>>) -> Result<(), Error> {
        let query = words.join(" ");

        let mut app = App::default();
        let mut search_menu = SearchMenu::new(query, server_list_lock.clone());

        app.run(&mut search_menu)?;
        Ok(())
    }

    fn process_stats(server_list_lock: Arc<RwLock<ServerList>>) -> Result<(), Error> {
        let server_list = server_list_lock.read();

        let stats = server_list.get_stats(server_list.get_default()?)?;
        
        println!("{stats}");
        Ok(())
    }

    fn process_whoami(server_list_lock: Arc<RwLock<ServerList>>) -> Result<(), Error> {
        match server_list_lock.read().get_default() {
            Ok(server) => {
                println!("You are on {}", server);
                println!("TODO: have the server store the current login info");
            },
            Err(_) => {
                // You DEFINITELY have none; it can't be outta bounds, right?!
                eprintln!("I couldn't get the current server... You might have none!");
            }
        }
        Ok(())
    }
}
