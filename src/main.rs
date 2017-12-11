

fn main() {
    println!("Hello, world!");
}

fn lexing<T: Iterator<Item=char>>(src: T) -> Result<Vec<Token>, String> {
    let mut ident = String::new();
    let mut ret = Vec::new();
    for c in src {
        if c == ' ' || c == '\n' {
            if ident.is_empty() {
                return Err("unexpected whitespace".to_string())
            } else {
                if ident == "quote" {
                    ret.push(Token::Quote)
                } else {
                    ret.push(Token::Ident(::std::mem::replace(&mut ident, String::new())))
                }
            }
        }
        else {
            ident.push(c)
        }
    }
    if ident.is_empty() {
        return Err("unexpected whitespace".to_string())
    }
    if ident == "quote" {
        ret.push(Token::Quote)
    } else {
        ret.push(Token::Ident(ident))
    }
    Ok(ret)
}

#[derive(Debug)]
enum Token {
    Quote,
    Ident(String),
}

impl<'a> ::std::cmp::PartialEq<&'a str>  for Token {
    fn eq(&self, rhs: &&str) -> bool {
        if let &Token::Ident(ref s) = self {
            return s == rhs
        }
        return false
    }
}

#[test]
fn test() {
    assert_eq!(lexing("you are fired".chars()).unwrap(), ["you", "are", "fired"])
}
