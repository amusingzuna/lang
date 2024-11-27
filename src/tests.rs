#[cfg(test)]
mod tests {
    use crate::parser::*;

    #[test]
    fn alternative_parsers() {
        let a_or_b_parser = char('a').or(char('b'));

        assert_eq!(a_or_b_parser.parse("abc"), Ok(('a', "bc")));
        assert_eq!(a_or_b_parser.parse("bbc"), Ok(('b', "bc")));
        assert_eq!(a_or_b_parser.parse("cbc"), Err("Character mismatch"));
    }

    #[test]
    fn applicative_parsers() {
        let a_and_b_parser = char('a').and(char('b'));

        assert_eq!(a_and_b_parser.parse("abc"), Ok((('a', 'b'), "c")));
        assert_eq!(a_and_b_parser.parse("aac"), Err("Character mismatch"));

        let a_left_b_parser = char('a').left(char('b'));

        assert_eq!(a_left_b_parser.parse("abc"), Ok(('a', "c")));

        let a_left_b_parser = char('a').right(char('b'));

        assert_eq!(a_left_b_parser.parse("abc"), Ok(('b', "c")));
    }

    #[test]
    fn parse_any() {
        assert_eq!(any().parse("abc"), Ok(('a', "bc")));
        assert_eq!(any().parse(""), Err("Nothing to parse"));
    }

    #[test]
    fn parse_char() {
        assert_eq!(char('a').parse("abc"), Ok(('a', "bc")));
        assert_eq!(char('a').parse("xyz"), Err("Character mismatch"));
        assert_eq!(char('a').parse(""), Err("Character mismatch"));
    }

    #[test]
    fn parse_pure() {
        assert_eq!(Parser::pure('a').parse("bc"), Ok(('a', "bc")));
        assert_eq!(Parser::pure('b').parse(""), Ok(('b', "")));
    }

    #[test]
    fn parse_impure() {
        assert_eq!(Parser::<()>::impure("failed").parse("abc"), Err("failed"))
    }

    #[test]
    fn parse_list() {
        let a_or_b_parser = list(&['a', 'b']);

        assert_eq!(a_or_b_parser.parse("abc"), Ok(('a', "bc")));
        assert_eq!(a_or_b_parser.parse("bbc"), Ok(('b', "bc")));
        assert_eq!(a_or_b_parser.parse("cbc"), Err("Character mismatch"));

        let empty_list = list(&[]);

        assert_eq!(empty_list.parse("abc"), Err("List parser has no members"));
    }
}
