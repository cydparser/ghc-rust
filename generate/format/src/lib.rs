/// Add blank lines to source code. This is necessary because neither _prettyplease_ nor _rustfmt_
/// adds vertical space in a useful way.
pub fn add_blank_lines(src: &str) -> String {
    let mut padded = String::with_capacity((src.len() as f64 * 1.05) as usize);
    let mut add_newline = false;
    let mut prev_context = Context::Blank;

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
                "pub" | "pub(crate)"
                    if !trimmed.starts_with("pub mod ") && !trimmed.starts_with("pub use ") =>
                {
                    if !add_newline && prev_context.permit_newline() && !trimmed.ends_with(",") {
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
                "const" | "static" if trimmed.ends_with(";") => (true, Context::Unknown),
                "if" | "loop" | "match" | "return" | "while"
                    if !add_newline && prev_context.permit_newline() =>
                {
                    padded.push('\n');
                    (false, Context::Unknown)
                }
                "//" | "///" => (false, Context::Comment),
                "mod" => maybe_add_newline(add_newline, prev_context, Context::Mod, &mut padded),
                _ if trimmed.starts_with("pub mod ") => {
                    maybe_add_newline(add_newline, prev_context, Context::Mod, &mut padded)
                }
                "use" => maybe_add_newline(add_newline, prev_context, Context::Use, &mut padded),
                _ if trimmed.starts_with("pub use ") => {
                    maybe_add_newline(add_newline, prev_context, Context::Use, &mut padded)
                }
                _ if trimmed == "}" || trimmed == "};" => (true, Context::Unknown),
                _ if trimmed.starts_with("#") => {
                    maybe_add_newline(add_newline, prev_context, Context::Macro, &mut padded)
                }
                _ if trimmed.ends_with("{") => (false, Context::Open),
                _ => (false, Context::Unknown),
            };

            padded.push_str(line);
            padded.push('\n');
        } else {
            if !add_newline && prev_context != Context::Blank {
                padded.push('\n');
            }
            add_newline = false;
            prev_context = Context::Blank;
        };
    }

    padded
}

#[derive(Clone, Copy, PartialEq)]
enum Context {
    Unknown,
    Blank,
    Comment,
    Macro,
    Mod,
    Open,
    Use,
}

impl Context {
    fn permit_newline(self) -> bool {
        !matches!(
            self,
            Context::Blank | Context::Comment | Context::Macro | Context::Open
        )
    }
}

fn maybe_add_newline(
    add_newline: bool,
    prev_context: Context,
    context: Context,
    padded: &mut String,
) -> (bool, Context) {
    if !add_newline && prev_context != context && prev_context.permit_newline() {
        padded.push('\n');
    }

    (false, context)
}
