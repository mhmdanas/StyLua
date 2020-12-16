use full_moon::tokenizer::{Token, TokenReference, TokenType};
use std::borrow::Cow;

fn format_eof<'ast>(_eof: TokenReference<'ast>) -> TokenReference<'ast> {
    TokenReference::new(
        Vec::new(),
        Token::new(TokenType::Eof),
        vec![Token::new(TokenType::Whitespace {
            characters: Cow::Owned(String::from("\n")),
        })],
    )
}

// #[cfg(test)]
// mod tests {
//     use crate::formatters::eof_formatter::EofFormatter;
//     use full_moon::visitors::VisitorMut;
//     use full_moon::{parse, print};
//     #[test]
//     fn test_eof_no_newline_formatter() {
//         let mut visitor = EofFormatter::default();
//         let ast = parse("local foo = 'bar'").unwrap();
//         assert_eq!(print(&visitor.visit_ast(ast)), "local foo = 'bar'\n");
//     }

//     #[test]
//     fn test_eof_too_many_newlines_formatter() {
//         let mut visitor = EofFormatter::default();
//         let ast = parse("local foo = 'bar'\n\n \n").unwrap();
//         assert_eq!(print(&visitor.visit_ast(ast)), "local foo = 'bar'\n");
//     }
// }
