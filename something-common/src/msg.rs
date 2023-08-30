use std::{default, fmt::Display};

use colored::{ColoredString, Colorize};
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Msg {
    pub header: ColoredString,
    pub msg_type: MsgType,
    pub subheader: Vec<ColoredString>,
    pub body: Vec<(Option<ColoredString>, ColoredString)>,
    pub body_margin: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum MsgType {
    #[default]
    Info,
    Warning,
    Error,
}
impl Msg {
    pub fn warning() -> Self {
        Self {
            msg_type: MsgType::Warning,
            ..Default::default()
        }
    }
    pub fn error() -> Self {
        Self {
            msg_type: MsgType::Error,
            ..Default::default()
        }
    }
    pub fn info() -> Self {
        Self {
            msg_type: MsgType::Info,
            ..Default::default()
        }
    }
    pub fn header(mut self, header: ColoredString) -> Self {
        self.header = header;
        self
    }
    pub fn subheader(mut self, subheader: Vec<ColoredString>) -> Self {
        self.subheader = subheader;
        self
    }
    pub fn push_body(mut self, body: ColoredString) -> Self {
        self.body.push((None, body));
        self
    }
    pub fn push_body_w_margin(
        mut self,
        body_text: ColoredString,
        text_on_margin: ColoredString,
    ) -> Self {
        self.body_margin = self.body_margin.max(text_on_margin.len() as u8 - 1);
        self.body.push((Some(text_on_margin), body_text));
        self
    }
}

impl Display for Msg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp = &self.header;
        write!(
            f,
            "{}",
            match self.msg_type {
                MsgType::Info => "Info: ".bright_blue(),
                MsgType::Warning => "Warning: ".yellow(),
                MsgType::Error => "Error: ".red(),
            }
            .bold()
        )?;
        writeln!(f, "{tmp}")?;
        for subheader in &self.subheader {
            writeln!(f, "{}{subheader}", "--> note: ".bright_cyan().bold())?;
        }
        // TODO
        for (margin, body) in &self.body {
            if let Some(margin) = margin {
                writeln!(f, "  {margin} | {body}",)?;
            } else {
                writeln!(f, "    | {body}", body = body)?;
            }
        }
        Ok(())
    }
}

#[test]
fn test() {
    let msg = Msg::error()
        .header("Deprecation".into())
        .subheader(vec!["Version 1.2.3".into(), "testing".into()])
        .push_body("...".into())
        .push_body_w_margin("let var = 134".into(), "1".into())
        .push_body("...".into());
    println!("{}", msg);
}
