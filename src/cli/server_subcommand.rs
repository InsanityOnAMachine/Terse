use super::*;
use text_io::read;

use crate::network::LoginOption;
use crate::network::ServerList;

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
                server_list.add_server(url.clone(), !stay);
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

                // THIS IS NOT TRUE
                // So we ask to sign in with this info;
                // And if the user is already verified and the password matches,
                // We're let in.
                // Otherwise the server creates the login info and asks us for the code.

                // THIS IS CURRENTLY TRUE
                // We ask to 'create an login info on the server'
                // I.e. "Server, trust that I own this email address!"
                // Well, the server don't trust us, it says, 'prove it!'
                // It sends us a code, and we use it as our login info passcode,
                // doing a quick check first to the server to ask, 'hey, did we do it right?'
                // and so on.

                match server_list.request_login(server_list.get_default()?, &login_info)? {
                    LoginOption::PleaseVerify => {
                        println!("The server hasn't seen that email before, so you'll need to verify it.");
                        println!("It should have sent a code to your inbox; please enter it here:");
                        let code: String = read!("{}\n");

                        if !server_list.verify_user(server_list.get_default()?, &login_info, &code)? {
                            println!("The code was incorrect, please try again.");
                            return Ok(())
                        }
                    },
                    LoginOption::BadPassword => {
                        println!("The server said that you have the wrong password for {}", &login_info.email)
                    }
                    _ => {}
                }

                server_list.get_mut_default()?.set_login_info(login_info.clone());
                println!("I successfully signed you into {} as {}", server_list.get_default()?, &login_info.email);
            }
        }
        Ok(())
    }
}