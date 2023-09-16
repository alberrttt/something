use std::fmt::Display;

use colored::{ColoredString, Colorize};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Log {
    pub header: Header,
    pub body: LogBody,
}
impl Log {
    pub fn new(header: Header, body: LogBody) -> Self {
        Self { header, body }
    }
}
impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.header)?;
        writeln!(f, "  {}", self.body)?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LogBody {
    pub body: Vec<BodyLine>,
}

impl LogBody {
    pub fn new(body: Vec<ColoredString>) -> Self {
        Self {
            body: body
                .into_iter()
                .map(|line| BodyLine { line, margin: None })
                .collect(),
        }
    }

    pub fn with_margin(body: Vec<BodyLine>) -> Self {
        Self { body }
    }
    pub fn push(mut self, line: impl Into<ColoredString>) -> Self {
        self.body.push(BodyLine {
            line: line.into(),
            margin: None,
        });
        self
    }
    pub fn push_margin(
        mut self,
        line: impl Into<ColoredString>,
        margin: impl Into<ColoredString>,
    ) -> Self {
        self.body.push(BodyLine {
            line: line.into(),
            margin: Some(margin.into()),
        });
        self
    }
}
impl Display for LogBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.body {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BodyLine {
    pub line: ColoredString,
    pub margin: Option<ColoredString>,
}
impl Display for BodyLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(margin) = &self.margin {
            write!(f, "{:0>4} | {}", margin, self.line)
        } else {
            write!(f, "\t {}", self.line)
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Header {
    text: Vec<ColoredString>,
}
impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for text in &self.text {
            write!(f, "{}", text)?;
        }
        Ok(())
    }
}
impl Header {
    pub fn push(mut self, text: impl Into<ColoredString>) -> Self {
        self.text.push(text.into());
        self
    }
    pub fn new(text: Vec<ColoredString>) -> Self {
        Self { text }
    }
    pub fn err(mut self) -> Self {
        self.text.push("err ".red().bold());
        self
    }
    pub fn warn(mut self) -> Self {
        self.text.push("warn ".yellow().bold());
        self
    }
    pub fn info(mut self) -> Self {
        self.text.push("info ".blue().bold());
        self
    }
}
#[test]
fn line() {
    let body_line = BodyLine {
        line: "hello".red(),
        margin: Some("3".blue()),
    };
    println!("{}", body_line);
}
#[test]
fn log() {
    let Log = Log::new(
        Header::default().err().push("test"),
        LogBody::default()
            .push_margin("hello world", "2")
            .push("hello"),
    );
    println!("{}", Log);
}
