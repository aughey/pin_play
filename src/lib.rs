enum StringOrStr<'a> {
    String(String),
    Str(&'a str),
}

impl<'a> StringOrStr<'a> {
    fn as_str(&self) -> &str {
        match self {
            StringOrStr::String(s) => s,
            StringOrStr::Str(s) => s,
        }
    }
}

fn parse<'a>(input: &'a str) -> StringOrStr<'a> {
    if input.starts_with("s:") {
        StringOrStr::Str(&input[2..])
    } else {
        StringOrStr::String(input.to_string())
    }
}

struct Parsed<'a> {
    value: StringOrStr<'a>,
}

impl<'a> Parsed<'a> {
    fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

fn outer_parse<'a>(input: &'a str) -> Parsed<'a> {
    let res = parse(input);
    Parsed { value: res }
}