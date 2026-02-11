/// Add blank lines to source code. This is necessary because neither _prettyplease_ nor _rustfmt_
/// adds vertical space in a useful way.
pub fn add_blank_lines(src: &str) -> String {
    let mut padded = String::with_capacity((src.len() as f64 * 1.05) as usize);
    let mut add_newline = false;
    let mut prev_context = Context::Unknown;

    for line in src.lines() {
        let trimmed = line.trim();

        if add_newline
            && !trimmed.ends_with('}')
            && !trimmed.ends_with("};")
            && !trimmed.contains(" => ")
        {
            padded.push('\n');
        }

        if let Some(token) = trimmed.split_whitespace().next() {
            (add_newline, prev_context) = match token {
                "pub" | "pub(crate)" => {
                    if !add_newline
                        && prev_context != Context::Macro
                        && prev_context.permit_newline()
                        && !trimmed.ends_with(",")
                    {
                        padded.push('\n');
                    }
                    (
                        trimmed.ends_with(";"),
                        if trimmed.ends_with("{") {
                            Context::Open
                        } else {
                            Context::Unknown
                        },
                    )
                }
                "const" | "mod" | "static" if trimmed.ends_with(";") => (true, Context::Unknown),
                "if" | "loop" | "match" | "return" | "while"
                    if !add_newline && prev_context.permit_newline() =>
                {
                    padded.push('\n');
                    (false, Context::Unknown)
                }
                "///" => (false, Context::Comment),
                "use" => {
                    if !add_newline && prev_context != Context::Use && prev_context.permit_newline()
                    {
                        padded.push('\n');
                    }

                    (false, Context::Use)
                }
                _ if trimmed == "}" || trimmed == "};" => (true, Context::Unknown),
                _ if trimmed.starts_with("#") => {
                    if !add_newline
                        && prev_context != Context::Macro
                        && prev_context.permit_newline()
                    {
                        padded.push('\n');
                    }

                    (false, Context::Macro)
                }
                _ if trimmed.ends_with("{") => (false, Context::Open),
                _ => (false, Context::Unknown),
            };

            padded.push_str(line);
        } else {
            add_newline = false;
            prev_context = Context::Blank;
        };

        padded.push('\n');
    }

    padded
}

#[derive(Clone, Copy, PartialEq)]
enum Context {
    Unknown,
    Blank,
    Comment,
    Macro,
    Open,
    Use,
}

impl Context {
    fn permit_newline(self) -> bool {
        self != Context::Blank && self != Context::Open && self != Context::Comment
    }
}
