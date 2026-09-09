use ratatui::prelude::{Stylize, Span, Line, Text};
use serde::{Serialize, Deserialize};

use std::fmt::{Display, Write};
use colored::Colorize;

use derive_new::new;

#[derive(Clone, Serialize, Deserialize, new)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Eq)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

impl<'a> From<&'a LoginInfo> for Text<'a> {
    fn from(value: &'a LoginInfo) -> Self {
        Text::from(vec![
            Line::from(Span::from(&value.email)).red(),
            Line::from(Span::from(&value.password)).blue(),
        ])
    }
}

impl Display for LoginInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // https://stackoverflow.com/questions/56612060/how-to-call-function-from-certain-trait-explicitly
        write!(f, "{}", self.as_string(false))?;
        Ok(())
    }
}

impl LoginInfo {
    pub fn as_string(&self, show_password: bool) -> String {
        let mut s = String::new();
        write!(s, "{}", Colorize::red(self.email.as_str())).expect("String writing should always work");
        if show_password {
           write!(s, " ({})", Colorize::blue(self.password.as_str())).expect("String writing should always work");
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_email_by_default() {
        assert_eq!(
            LoginInfo::new("email".into(), "password".into()).to_string(),
            format!("{}", Colorize::red("email"))
        )
    }

    #[test]
    fn displays_no_password_if_specified() {
        assert_eq!(
            LoginInfo::new("email".into(), "password".into()).as_string(false),
            format!("{}", Colorize::red("email"))
        )
    }

    #[test]
    fn displays_password_if_required() {
        assert_eq!(
            LoginInfo::new("email".into(), "password".into()).as_string(true),
            format!("{} ({})", Colorize::red("email"), Colorize::blue("password"))
        )
    }
}