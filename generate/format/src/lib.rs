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
            && !trimmed.starts_with("use ")
            && !trimmed.contains(" => ")
        {
            padded.push('\n');
        }

        let sans_vis = trimmed
            .starts_with("pub")
            .then(|| {
                trimmed
                    .strip_prefix("pub ")
                    .or_else(|| trimmed.strip_prefix("pub(crate) "))
            })
            .flatten()
            .unwrap_or(trimmed);

        if let Some(token) = sans_vis.split_whitespace().next() {
            (add_newline, prev_context) = match token {
                _ if sans_vis.ends_with(',') => (false, Context::Unknown),
                "const" | "enum" | "static" | "struct" | "type" => {
                    if !add_newline && prev_context.permit_newline() {
                        padded.push('\n');
                    }
                    (sans_vis.ends_with(";"), Context::Unknown)
                }
                "if" | "loop" | "match" | "return" | "while"
                    if !add_newline && prev_context.permit_newline() =>
                {
                    padded.push('\n');
                    (false, Context::Unknown)
                }
                "//" | "///" => {
                    maybe_add_newline(add_newline, prev_context, Context::Comment, &mut padded)
                }
                "mod" => {
                    if sans_vis.ends_with(';') {
                        (true, Context::Mod)
                    } else {
                        (false, Context::Open)
                    }
                }
                "use" => maybe_add_newline(add_newline, prev_context, Context::Use, &mut padded),
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
