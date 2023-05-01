use something_frontend_tokenizer::{Parse, Tokens};

// prolly need better error handling soon
#[derive(Debug, Clone)]
pub struct Punctuated<T, P>(pub Vec<(T, Option<P>)>);
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
