use std::sync::Arc;

pub struct Parser<'a, T: 'a>(Arc<dyn Fn(&'a str) -> Result<(T, &'a str), &'static str> + 'a>);

impl<'a, T: 'a> Clone for Parser<'a, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'a, T> Parser<'a, T> {
    pub fn new<F>(parser: F) -> Self
    where
        F: 'a + Fn(&'a str) -> Result<(T, &'a str), &'static str>,
    {
        Self(Arc::new(parser))
    }

    pub fn lazy<F>(parser_factory: F) -> Self
    where
        F: Fn() -> Self + 'a,
    {
        Parser::new(move |input: &'a str| {
            let parser = parser_factory();
            parser.parse(input)
        })
    }

    pub fn map<U, F>(self, func: F) -> Parser<'a, U>
    where
        F: 'a + Fn(T) -> U,
        U: 'a,
    {
        Parser::new(move |input: &'a str| {
            self.parse(input)
                .map(|(result, remaining)| (func(result), remaining))
        })
    }

    pub fn pure(a: T) -> Self
    where
        T: 'a + Clone,
    {
        Parser::new(move |input: &'a str| Ok((a.clone(), input)))
    }

    pub fn many(self) -> Parser<'a, Vec<T>> {
        Parser::new(move |mut input: &'a str| {
            let mut results = Vec::new();
            while let Ok((result, remaining)) = self.parse(input) {
                results.push(result);
                input = remaining;
            }
            Ok((results, input))
        })
    }

    pub fn some(self) -> Parser<'a, Vec<T>> {
        self.clone().and(self.many()).map(|(head, rest)| {
            let mut result = Vec::with_capacity(rest.len() + 1);
            result.push(head);
            result.extend(rest);
            result
        })
    }

    pub fn and<U>(self, other: Parser<'a, U>) -> Parser<'a, (T, U)>
    where
        U: 'a,
    {
        Parser::new(move |input: &'a str| {
            self.parse(input).and_then(|(result_a, remaining_a)| {
                other
                    .parse(remaining_a)
                    .map(|(result_b, remaining_b)| ((result_a, result_b), remaining_b))
            })
        })
    }

    pub fn left<U>(self, other: Parser<'a, U>) -> Parser<'a, T>
    where
        U: 'a,
    {
        let merge = self.and(other);

        Parser::new(move |input: &'a str| match merge.parse(input) {
            Ok(((left, _), remaining)) => Ok((left, remaining)),
            Err(reason) => Err(reason),
        })
    }

    pub fn right<U>(self, other: Parser<'a, U>) -> Parser<'a, U>
    where
        U: 'a,
    {
        let merge = self.and(other);

        Parser::new(move |input: &'a str| match merge.parse(input) {
            Ok(((_, right), remaining)) => Ok((right, remaining)),
            Err(reason) => Err(reason),
        })
    }

    pub fn empty(reason: &'static str) -> Self {
        Parser::new(move |_| Err(reason))
    }

    pub fn or(self, other: Parser<'a, T>) -> Parser<'a, T> {
        Parser::new(move |input: &'a str| self.parse(input).or_else(|_| other.parse(input)))
    }

    pub fn parse(&self, input: &'a str) -> Result<(T, &'a str), &'static str> {
        (self.0)(input)
    }
}

impl<'a> Parser<'a, Vec<char>> {
    pub fn qualify(self) -> Parser<'a, String> {
        self.map(|x| x.into_iter().collect())
    }
}

pub fn any<'a>() -> Parser<'a, char> {
    Parser::new(move |input: &'a str| {
        let mut chars = input.chars();
        match chars.next() {
            Some(c) => Ok((c, chars.as_str())),
            _ => Err("Nothing to parse"),
        }
    })
}

pub fn char<'a>(expected: char) -> Parser<'a, char> {
    Parser::new(move |input: &'a str| {
        let mut chars = input.chars();
        match chars.next() {
            Some(c) if c == expected => Ok((c, chars.as_str())),
            _ => Err("Character mismatch"),
        }
    })
}

pub fn list<'a>(allowed: &[char]) -> Parser<'a, char> {
    let fail = Parser::empty("List parser has no members");
    allowed.iter().fold(fail, |sum, x| sum.or(char(*x)))
}

pub fn whitespace<'a>() -> Parser<'a, char> {
    list(&[' ', '\n', '\t', '\r'])
}

pub fn lowercase<'a>() -> Parser<'a, char> {
    list(&[
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ])
}

pub fn uppercase<'a>() -> Parser<'a, char> {
    list(&[
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ])
}

pub fn letter<'a>() -> Parser<'a, char> {
    lowercase().or(uppercase()).or(char('_'))
}

pub fn digit<'a>() -> Parser<'a, char> {
    list(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
}

pub fn alphanumeric<'a>() -> Parser<'a, char> {
    letter().or(digit())
}

pub fn between<'a, T, U, P>(a: Parser<'a, T>, b: Parser<'a, U>, c: Parser<'a, P>) -> Parser<'a, U> {
    a.right(b).left(c)
}

pub fn otherwise<'a, T: Clone>(b: Parser<'a, T>, a: T) -> Parser<'a, T> {
    b.or(Parser::pure(a))
}

pub fn delimited<'a, T: Clone, U>(a: Parser<'a, T>, sep: Parser<'a, U>) -> Parser<'a, Vec<T>> {
    otherwise(
        a.clone().and(sep.right(a).many()).map(|(first, mut rest)| {
            rest.insert(0, first);
            rest
        }),
        vec![],
    )
}

pub fn strip<'a, T>(a: Parser<'a, T>) -> Parser<'a, T> {
    between(whitespace().many(), a, whitespace().many())
}

pub fn string<'a>(input: &'a str) -> Parser<'a, String> {
    if input.is_empty() {
        return Parser::pure("".to_string());
    }

    char(input.chars().next().unwrap())
        .and(string(&input[1..]))
        .map(|(c, rest)| {
            let mut result = "".to_string();
            result.push(c);
            result.push_str(&rest);
            result
        })
}

pub fn identifier<'a>() -> Parser<'a, String> {
    strip(letter().and(alphanumeric().many()).map(|(first, rest)| {
        let mut result = String::new();
        result.push(first);
        result.push_str(&rest.iter().collect::<String>());
        result
    }))
}

pub fn symbol<'a>(a: &'a str) -> Parser<'a, String> {
    strip(string(a))
}

pub fn tuple<'a, T>(a: Parser<'a, T>) -> Parser<'a, T> {
    between(symbol("("), a, symbol(")"))
}

pub fn set<'a, T>(a: Parser<'a, T>) -> Parser<'a, T> {
    between(symbol("["), a, symbol("]"))
}

pub fn block<'a, T>(a: Parser<'a, T>) -> Parser<'a, T> {
    between(symbol("{"), a, symbol("}"))
}

pub fn integer<'a>() -> Parser<'a, String> {
    strip(digit().some().map(|c| c.into_iter().collect()))
}

pub fn float<'a>() -> Parser<'a, String> {
    strip(
        digit()
            .some()
            .qualify()
            .and(symbol("."))
            .and(digit().many().qualify())
            .left(symbol("f").many())
            .map(|((a, b), c)| {
                let mut result = String::new();
                result.push_str(a.as_str());
                result.push_str(b.as_str());
                result.push_str(c.as_str());
                result
            }),
    )
}
