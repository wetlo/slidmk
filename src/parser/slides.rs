use super::{
    lexer::Lexer,
    slide::{Content, Slide},
    tokens::Token,
};
use crate::util::RemoveFirst;

pub struct Slides<I>
where
    I: Iterator<Item = char>,
{
    tokens: RemoveFirst<Lexer<I>>,
    next_token: Option<Token>,
}

impl<I> Slides<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(source: I) -> Self {
        Self {
            tokens: RemoveFirst::new(Lexer::new(source), Token::Linefeed),
            next_token: None,
        }
    }

    fn buf_next(&mut self) -> Option<Self::Item> {
        self.next_token.take().or_else(|| self.tokens.next())
    }

    fn get_kind(&mut self) -> Option<String> {
        if let Some(Token::Identifier(s)) = self.buf_next() {
            Some(s)
        } else {
            None
        }
    }

    fn concat_text(&mut self, coll: String) -> String {
        loop {
            let next = self.next();
            if let Some(Token::Text(s)) = next {
                coll.push_str(&s);
            } else {
                self.next_token = next;
                break;
            }
        }

        coll
    }

    fn get_list(&mut self, first_ident: u8) -> Option<Vec<(u8, String)>> {
        todo!()
    }

    fn get_content(&mut self) -> Option<Content> {
        use Token::*;

        match self.buf_next()? {
            Text(s) => Content::Text(self.concat_text(s)),
            Path(p) => Content::Path(p),
            t => {
                self.next_token = Some(t);
                None
            }
        }
    }
}

impl<I> Iterator for Slides<I>
where
    I: Iterator<Item = char>,
{
    type Item = Slide;
    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.get_kind()?;
        None
    }
}
