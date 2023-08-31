use std::{default, fmt::Display};

use colored::{ColoredString, Colorize};
use pad::{Alignment, PadStr};
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
    pub fn header(mut self, header: impl Into<ColoredString>) -> Self {
        self.header = header.into();
        self
    }
    pub fn subheader(mut self, subheader: Vec<ColoredString>) -> Self {
        self.subheader = subheader;
        self
    }
    pub fn push_body(mut self, body: impl Into<ColoredString>) -> Self {
        self.body.push((None, body.into()));
        self
    }
    pub fn push_body_w_margin(
        mut self,
        body_text: ColoredString,
        text_on_margin: ColoredString,
    ) -> Self {
        self.body_margin = self.body_margin.max(text_on_margin.len() as u8 - 1);
        self.body.push((
            Some(match self.msg_type {
                MsgType::Info => text_on_margin.bright_blue(),
                MsgType::Warning => text_on_margin.yellow(),
                MsgType::Error => text_on_margin.red(),
            }),
            body_text,
        ));
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
                MsgType::Info => "info ".bright_blue(),
                MsgType::Warning => "warning ".yellow(),
                MsgType::Error => "error ".red(),
            }
            .bold()
        )?;
        writeln!(f, "{tmp}")?;
        for subheader in &self.subheader {
            writeln!(f, "{}{subheader}", "--> note: ".bright_cyan().bold())?;
        }
        // TODO
        writeln!(f);
        for (margin, body) in &self.body {
            let red_bar = "|".red().to_string();
            let margin_text = match margin {
                Some(margin) => margin
                    .pad_to_width_with_alignment(self.body_margin as usize + 1, Alignment::Right),
                None => "".pad_to_width(self.body_margin as usize + 1),
            }
            .red()
            .to_string();
            writeln!(f, " {margin_text} {}\t{body}", red_bar, body = body,)?;
        }
        Ok(())
    }
}

#[test]
fn test() {
    let msg = Msg::error()
        .header("Deprecation")
        .push_body("...")
        .push_body_w_margin("let var = 134".into(), "1".into())
        .push_body_w_margin("print(hello world)".into(), "12".into())
        .push_body_w_margin("print(hello world)".into(), "1233".into())
        .push_body("...");
    println!("{}", msg);
}
