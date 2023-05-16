use std::fmt::Display;

use something_frontend_tokenizer::{Parse, ParsingDisplay, Tokens};

// prolly need better error handling soon
#[derive(Debug, Clone)]
pub struct Punctuated<T, P>(pub Vec<(T, Option<P>)>);

impl<T, P> ParsingDisplay for Punctuated<T, P>
where
    T: ParsingDisplay,
    P: ParsingDisplay,
{
    fn display(&self) -> String {
        self.0
            .iter()
            .map(|(t, p)| {
                let mut tmp = t.display();
                if let Some(p) = p {
                    tmp.push_str(&p.display());
                }
                tmp
            })
            .collect::<String>()
    }
    fn placeholder() -> String {
        todo!()
    }
}
impl<T, P> Parse for Punctuated<T, P>
where
    T: Parse,
    P: Parse,
{
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
        Self::parse_terminated(input)
    }
}
impl<T, P> Punctuated<T, P>
where
    T: Parse,
    P: Parse,
{
    pub fn has_trailing(&self) -> bool {
        self.0.last().unwrap().1.is_some()
    }
    pub fn parse_without_trailing(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
        todo!();
        let mut vec = Vec::new();
        loop {
            if input.at_end() || input.is_empty() {
                break;
            }
            let item = T::parse(input)?;
            vec.push((item, None));
        }
        Ok(Self(vec))
    }
    pub fn parse_trailing(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
        let mut vec = Vec::new();
        loop {
            if input.at_end() || input.is_empty() {
                break;
            }
            let item = T::parse(input)?;

            vec.push((item, Some(P::parse(input)?)));
        }
        Ok(Self(vec))
    }
    pub fn parse_terminated(
        input: &mut something_frontend_tokenizer::Tokens,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut vec = Vec::new();
        loop {
            if input.at_end() || input.is_empty() {
                break;
            }
            let item = T::parse(input)?;
            let punct = if input.is_empty() {
                None
            } else {
                let parse = input.step(|f| P::parse(f));
                match parse {
                    Ok(punct) => Some(punct),
                    Err(err) => {
                        if input.distance_from_end() == 0 {
                            vec.push((item, None));
                            break;
                        } else {
                            return Err(err);
                        }
                    }
                }
            };
            vec.push((item, punct));
        }
        Ok(Self(vec))
    }
}
