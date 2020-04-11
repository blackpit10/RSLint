use super::{
  lexer::Lexer,
  token::{TokenType, Token},
  util::CharExt,
};
use crate::keyword_trie;

//unique es5 tokens, only reserved in strict mode
static ES5_RESERVED_TOKENS: [TokenType; 2] = [
  TokenType::DeclarationLet,
  TokenType::Await
];

impl<'a> Lexer<'a> {

  /*
  * Resolve a sequence which can either be an identifier or a keyword
  * Matching uses a short circuited trie to be as fast as possible
  * The characters have to exactly match sequentially or it will be resolved as an identifier
  */
  pub fn resolve_ident_or_keyword(&mut self, ident_start: char) -> Token {
    let start = self.cur;
    if !ident_start.is_ascii_lowercase() { return self.resolve_identifier(start); }
    keyword_trie!(self, ident_start, start, {
      'b' => {
        'r' => Break,
      },
      'c' => {
        'a' => {
          's' => Case,
          't' => Catch,
        },
        'l' => Class,
        'o' => {
            'n' => {
                't' => Continue,
            },
        },
      },
      'd' => {
        'e' => {
          'b' => Debugger,
          'f' => Default,
          'l' => Delete,
        },
        'o' => Do,
      },
      'e' => {
        'l' => Else,
        'n' => Enum,
        'x' => {
          'p' => Export,
          't' => Extends,
        },
      },
      'f' => {
        'a' => False,
        'i' => Finally,
        'o' => For,
        'u' => Function,
      },
      'i' => {
        'f' => If,
        'm' => {
          'p' => {
            'l' => Implements,
            'o' => Import,
          },
        },
        'n' => In,
        'n' => {
          's' => Instanceof,
          't' => Interface,
        },
      },
      'n' => {
        'e' => New,
        'u' => Null,
      },
      'p' => {
        'a' => Package,
        'r' => {
          'i' => Private,
          'o' => Protected,
        },
        'u' => Public,
      },
      'r' => Return,
      's' => {
        't' => Static,
        'u' => Super,
        'w' => Switch,
      },
      't' => {
        'h' => {
          'i' => This,
          'r' => Throw,
        },
        'r' => Try,
        'y' => Typeof,
      },
      'v' => Void,
      'w' => {
        'h' => While,
        'i' => With,
      },
      'y' => Yield,
      }
    )
  }

  fn resolve_keyword(&mut self, expected: TokenType, start: usize, rest: &str) -> Token {
    for i in rest[self.cur - start + 1..].chars() {
      match self.source_iter.peek() {
        Some(c) if c.1 == i => { self.advance(); },
        Some(c) if c.1 != i && c.1.is_identifier_part() => return self.resolve_identifier(start),
        Some(_) => return self.token(start, TokenType::Identifier),
        None => return self.resolve_identifier(start)
      }
    }

    match self.source_iter.peek() {
      Some(c) if !c.1.is_identifier_part() => self.token(start, expected),
      Some(_) => self.resolve_identifier(start),
      None => self.token(start, expected)
    }
  }

  // Resolves a sequence determined to be an identifier into an identifier token
  fn resolve_identifier(&mut self, start: usize) -> Token {
    loop {
      match self.source_iter.peek() {
        Some(c) if c.1.is_identifier_part() => { self.advance(); },
        Some(c) if !c.1.is_identifier_part() => return self.token(start, TokenType::Identifier),
        Some(_) => return self.token(start, TokenType::Identifier),
        None => return self.token(start, TokenType::Identifier)
      }
    }
  }
}

#[cfg(test)]
mod test {
  use crate::parse::lexer::{
    lexer::Lexer,
    token::*,
    token::TokenType::*,
  };

  macro_rules! tok {
    ($type:ident) => {
      (TokenType::$type, stringify!($type).to_ascii_lowercase())
    };
  }

  // #[test]
  // fn identifiers() {
  //   let source = String::from("

  //   ")
  //   let lexer = Lexer::new()
  // }
}