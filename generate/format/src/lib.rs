/// Add blank lines to source code. This is necessary because neither _prettyplease_ nor _rustfmt_
/// adds vertical space in a useful way.
pub fn add_blank_lines(src: &str) -> String {
    let mut padded = String::with_capacity((src.len() as f64 * 1.05) as usize);
    let mut prev_context = Context::Blank;

    fn open_unless(p: bool) -> (bool, Context) {
        (true, if p { Context::Other } else { Context::Open })
    }

    fn newline_unless_same(prev_context: Context, context: Context) -> (bool, Context) {
        (prev_context != context, context)
    }

    for line in src.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            // Preserve at most one blank line.
            if prev_context.permit_newline() {
                padded.push('\n');
            }
            prev_context = Context::Blank;

            continue;
        }

        let sans_vis = {
            trimmed
                .strip_prefix("pub")
                .and_then(|s| {
                    s.split_once(' ').and_then(|(pre, rest)| {
                        (pre.is_empty() || pre.starts_with('(')).then(|| rest.trim())
                    })
                })
                .unwrap_or(trimmed)
        };

        let first_char = trimmed.chars().next().unwrap();
        let last_char = trimmed.chars().last().unwrap();

        let is_open = || matches!(last_char, '{' | '(' | '[' | '<');

        let (add_newline, context) = match sans_vis.split_whitespace().next().unwrap() {
            "const" | "static" | "type" => open_unless(last_char == ';'),
            "struct" => open_unless(matches!(last_char, ';' | '}')),
            "enum" | "extern" | "fn" | "if" | "impl" | "loop" | "match" | "trait" | "while" => {
                open_unless(last_char == '}')
            }
            "let" if prev_context != Context::Let => (true, Context::Let),
            "let" => {
                let is_open = is_open();
                (is_open, if is_open { Context::Open } else { Context::Let })
            }
            "unsafe" => {
                let is_open = !(last_char == '}' || trimmed.ends_with("};"));
                (
                    is_open,
                    if is_open {
                        Context::Open
                    } else {
                        Context::Other
                    },
                )
            }
            "return" => (true, Context::Other),
            "mod" => newline_unless_same(prev_context, Context::Mod),
            "as" | "use" => (
                false,
                if last_char == ';' {
                    Context::Other
                } else {
                    Context::Open
                },
            ),
            "}" | ")" | "]" if last_char == ';' => (false, Context::Close),
            _ if first_char == '/' => newline_unless_same(prev_context, Context::Comment),
            _ if first_char == '#' => newline_unless_same(prev_context, Context::Macro),
            _ if trimmed.starts_with("assert_") => (false, Context::Other),
            _ => match trimmed {
                "}" | "};" | ")" | ");" | "]" | "];" => (false, Context::Close),
                _ if last_char == ',' => (false, Context::Comma),
                _ if first_char == '.' || trimmed.contains("=>") => (false, Context::Other),
                _ if is_open() => (true, Context::Open),
                _ => (matches!(prev_context, Context::Close), Context::Other),
            },
        };

        if add_newline && prev_context.permit_newline() {
            padded.push('\n');
        }

        padded.push_str(line);
        padded.push('\n');

        prev_context = if context != Context::Open && is_open() {
            Context::Open
        } else {
            context
        };
    }

    padded
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Context {
    Blank,
    Close,
    Comma,
    Comment,
    Let,
    Macro,
    Mod,
    Open,
    Other,
}

impl Context {
    fn permit_newline(self) -> bool {
        !matches!(
            self,
            Context::Blank | Context::Comma | Context::Comment | Context::Macro | Context::Open
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_blank_lines() {
        let initial = "\
use foo::{
    bar,
};
pub use crate::*;
#[cfg(tests)]
mod tests;
type Foo = ();
struct S {
    s: (),
}
enum Void {}
const C: () = ();
static S: [(); 1] = [
    (),
];
pub unsafe extern \"C\" fn f() {
    use f;
    fn local() {
    }
    if true {
    }
    let a = ();
    let b = ();
    let c = [
        (),
    ];
    match c {
        [] => {
            if false {
            }
        }
        _ => (),
    }
    f();
    g()
        .h();
    j(
        1,
        2,
    );
    return ();
}\n";

        assert_eq!(
            add_blank_lines(initial),
            "\
use foo::{
    bar,
};
pub use crate::*;

#[cfg(tests)]
mod tests;

type Foo = ();

struct S {
    s: (),
}

enum Void {}

const C: () = ();

static S: [(); 1] = [
    (),
];

pub unsafe extern \"C\" fn f() {
    use f;

    fn local() {
    }

    if true {
    }

    let a = ();
    let b = ();

    let c = [
        (),
    ];

    match c {
        [] => {
            if false {
            }
        }
        _ => (),
    }

    f();
    g()
        .h();

    j(
        1,
        2,
    );

    return ();
}\n"
        );
    }
}
