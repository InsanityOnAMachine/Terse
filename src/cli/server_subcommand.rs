use super::*;
use text_io::read;

use clap::Parser;

use crate::network::LoginOption;
use crate::network::ServerList;

use crate::tui::{App, ServerMenu};

// https://stackoverflow.com/questions/71991935/how-to-make-a-default-subcommand-with-clap-and-derive
// We need this so that the whole subcommand can be optional
#[derive(Parser)]
pub struct ServerSubcommandHolder {
    #[command(subcommand)]
    command: Option<ServerSubcommand>,
}

impl ServerSubcommandHolder {
    pub fn process(self, server_list_lock: Arc<RwLock<ServerList>>) -> Result<(), Error> {
       match self.command {
            Some(command) => command.process(server_list_lock),
            // Figuring out that I needed this 'and' was actually the first useful use of Terse. Documented here.
            None => App::default().run(&mut ServerMenu::new(server_list_lock)).and(Ok(())),
        }
    }
}

#[derive(Subcommand)]
pub enum ServerSubcommand {
    #[command(override_help = include_str!("../docs/server/add.txt"))]
    Add {url: Url, #[arg(short)] stay: bool},
    #[command(override_help = include_str!("../docs/server/remove.txt"))]
    Remove {url: Url},
    #[command(override_help = include_str!("../docs/server/set.txt"))]
    Set {idx: usize},
    #[command(override_help = include_str!("../docs/server/login.txt"))]
    Login {email: String, password: String},
    #[command(override_help = include_str!("../docs/server/list.txt"))]
    // https://stackoverflow.com/questions/60458705/how-do-i-specify-a-boolean-command-line-flag-using-clap
    List {#[clap(short('p'), action)] show_passwords: bool},
}

impl ServerSubcommand {
    pub fn process(self, server_list_lock: Arc<RwLock<ServerList>>) -> Result<(), Error> {
        let mut server_list = server_list_lock.write();

        match self {
            Self::Add { url, stay } => {
                server_list.add_server(url.clone(), !stay)?;
                println!("I successfully added {url} to the list of servers!");
            }
            Self::Remove { url } => {
                server_list.remove_server(url.clone())?;
                println!("I successfully removed {} from the list of servers", url);
            }
            Self::Set { idx } => {
                let server = server_list.set_server(idx)?;
                // If this wanted more stats... I'd just not globally error;
                // I'd say "I successfully set the server to 4: [couldn't get name]""
                println!("I successfully set the server to {idx}: {}", server);
            }
            Self::List { show_passwords } => {
                println!("{}", server_list.as_string(show_passwords));
            }
            Self::Login { email, password } => {
                let login_info = LoginInfo::new(email, password);

                // We ask the server if we can login with this info

                match server_list.request_login(server_list.get_default()?, &login_info)? {
                    // 'This is a new email, so give me the code I sent to your email'
                    LoginOption::PleaseVerify => {
                        println!("The server hasn't seen that email before, so you'll need to verify it.");
                        println!("It should have sent a code to your inbox; please enter it here:");
                        let code: String = read!("{}\n");
                        if !server_list.verify_user(server_list.get_default()?, &login_info, &code)? {
                            println!("The code was incorrect, please try again.");
                            return Ok(())
                        }
                    },
                    // 'Wrong password!'
                    LoginOption::BadPassword => {
                        println!("The server said that you have the wrong password for {}", &login_info.email);
                        return Ok(())
                    }
                    _ => {}
                }
                // 'all OK!'
                server_list.get_mut_default()?.set_login_info(login_info.clone());
                println!("I successfully signed you into {} as {}", server_list.get_default()?.url_string(), &login_info.email);
            }
        }
        Ok(())
    }
}
