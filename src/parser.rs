pub struct Parser<'input, T>(
    Box<dyn Fn(&'input str) -> Result<(T, &'input str), &'static str> + 'input>,
)
where
    T: 'input;

impl<'input, T> Parser<'input, T> {
    pub fn new<F>(parser: F) -> Self
    where
        F: 'input + Fn(&'input str) -> Result<(T, &'input str), &'static str>,
    {
        Self(Box::new(parser))
    }

    pub fn pure(a: T) -> Self
    where
        T: Clone + Copy,
    {
        Parser::new(move |input: &'input str| Ok((a, input)))
    }

    pub fn many(self) -> Parser<'input, Vec<T>> {
        Parser::new(move |mut input: &'input str| {
            let mut results = Vec::new();
            while let Ok((result, remaining)) = self.parse(input) {
                results.push(result);
                input = remaining;
            }
            Ok((results, input))
        })
    }

    pub fn and<U>(self, other: Parser<'input, U>) -> Parser<'input, (T, U)>
    where
        U: 'input,
    {
        Parser::new(move |input: &'input str| {
            self.parse(input).and_then(|(result_a, remaining_a)| {
                other
                    .parse(remaining_a)
                    .map(|(result_b, remaining_b)| ((result_a, result_b), remaining_b))
            })
        })
    }

    pub fn left<U>(self, other: Parser<'input, U>) -> Parser<'input, T>
    where
        U: 'input,
    {
        let merge = self.and(other);

        Parser::new(move |input: &'input str| match merge.parse(input) {
            Ok(((left, _), remaining)) => Ok((left, remaining)),
            Err(reason) => Err(reason),
        })
    }

    pub fn right<U>(self, other: Parser<'input, U>) -> Parser<'input, U>
    where
        U: 'input,
    {
        let merge = self.and(other);

        Parser::new(move |input: &'input str| match merge.parse(input) {
            Ok(((_, right), remaining)) => Ok((right, remaining)),
            Err(reason) => Err(reason),
        })
    }

    pub fn empty(reason: &'static str) -> Self {
        Parser::new(move |_| Err(reason))
    }

    pub fn or(self, other: Parser<'input, T>) -> Parser<'input, T> {
        Parser::new(move |input: &'input str| self.parse(input).or_else(|_| other.parse(input)))
    }

    pub fn parse(&self, input: &'input str) -> Result<(T, &'input str), &'static str> {
        (self.0)(input)
    }
}

pub fn any() -> Parser<'static, char> {
    Parser::new(move |input: &'static str| {
        let mut chars = input.chars();
        match chars.next() {
            Some(c) => Ok((c, chars.as_str())),
            _ => Err("Nothing to parse"),
        }
    })
}

pub fn char(expected: char) -> Parser<'static, char> {
    Parser::new(move |input: &'static str| {
        let mut chars = input.chars();
        match chars.next() {
            Some(c) if c == expected => Ok((c, chars.as_str())),
            _ => Err("Character mismatch"),
        }
    })
}

pub fn list(allowed: &[char]) -> Parser<'static, char> {
    let fail = Parser::empty("List parser has no members");
    allowed.iter().fold(fail, |sum, x| sum.or(char(*x)))
}

pub fn whitespace() -> Parser<'static, char> {
    list(&[' ', '\n', '\t', '\r'])
}

pub fn lowercase() -> Parser<'static, char> {
    list(&[
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ])
}

pub fn uppercase() -> Parser<'static, char> {
    list(&[
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ])
}

pub fn letter() -> Parser<'static, char> {
    lowercase().or(uppercase())
}

pub fn digit() -> Parser<'static, char> {
    list(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
}

pub fn alphanumeric() -> Parser<'static, char> {
    letter().or(digit())
}

pub fn between<T>(
    a: Parser<'static, T>,
    b: Parser<'static, T>,
    c: Parser<'static, T>,
) -> Parser<'static, T> {
    return a.right(b).left(c);
}

pub fn option<T: Clone + Copy>(a: T, b: Parser<'static, T>) -> Parser<'static, T> {
    b.or(Parser::pure(a))
}
