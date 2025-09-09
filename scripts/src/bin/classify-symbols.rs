use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::Command;

use build_utils as utils;

pub fn main() {
    let ghc = utils::GhcDirs::new();

    let buf = {
        let mut buf = vec![];
        let bindings = ghc.rts_bindings(false);

        bindings.write(Box::new(&mut buf)).unwrap();

        buf
    };

    let visitor = {
        let mut visitor = SymbolVisitor::default();

        let code = std::str::from_utf8(&buf).unwrap();

        let file = syn::parse_file(code).unwrap();

        syn::visit::visit_file(&mut visitor, &file);

        visitor
    };

    dbg!(&visitor);

    let ghc_path = &ghc.root_dir;

    let mut symbols = BTreeMap::new();

    for sym in visitor.symbols {
        let places = find_places(ghc_path, &sym);
        symbols.insert(sym, places);
    }

    println!("#[derive(Clone, Copy, Debug)]\npub enum Place {{");
    for v in PLACE_VARIANTS {
        println!("{v:?} = 0b{:05b},", v as u32);
    }
    println!("}}\n");

    println!(
        "pub(crate) static PLACE_VARIANTS: [Place; {}] = [",
        PLACE_VARIANTS.len()
    );
    for v in PLACE_VARIANTS {
        println!("    Place::{v:?},");
    }
    println!("];\n");

    println!("#[derive(Clone, Copy, Default)]\npub struct Places(u32);\n");

    print_static_array("SYMBOLS", "(&str, Places)", symbols.len(), symbols);
    println!();
    print_static_array(
        "POINTER_TYPES",
        "&str",
        visitor.pointer_types.len(),
        visitor.pointer_types,
    );
    println!();
    print_static_array(
        "PRIMITIVE_TYPES",
        "&str",
        visitor.primitive_types.len(),
        visitor.primitive_types,
    );
}

#[derive(Debug, Default)]
struct SymbolVisitor {
    symbols: Vec<String>,
    pointer_types: BTreeSet<String>,
    primitive_types: BTreeSet<String>,
}

impl SymbolVisitor {
    fn add_symbol(&mut self, sym: String) {
        if sym != "_" {
            self.symbols.push(sym);
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Place {
    Compiler = 0b0001,
    Docs = 0b00010,
    Libraries = 0b00100,
    Testsuite = 0b01000,
    Utils = 0b10000,
}

static PLACE_VARIANTS: [Place; 5] = [
    Place::Compiler,
    Place::Docs,
    Place::Libraries,
    Place::Testsuite,
    Place::Utils,
];

struct Places(u32);

impl Places {
    fn new() -> Self {
        Places(0)
    }

    fn insert(&mut self, place: Place) {
        self.0 |= place as u32;
    }

    fn union(mut self, other: Places) -> Places {
        self.0 |= other.0;
        self
    }
}

impl std::fmt::Debug for Places {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Places(0b{:05b})", self.0)
    }
}

impl<'ast> syn::visit::Visit<'ast> for SymbolVisitor {
    fn visit_foreign_item_fn(&mut self, i: &'ast syn::ForeignItemFn) {
        self.add_symbol(i.sig.ident.to_string());
    }

    fn visit_foreign_item_static(&mut self, i: &'ast syn::ForeignItemStatic) {
        self.add_symbol(i.ident.to_string());
    }

    fn visit_item_const(&mut self, i: &'ast syn::ItemConst) {
        self.add_symbol(i.ident.to_string());
    }

    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        self.add_symbol(i.ident.to_string());
    }

    fn visit_item_static(&mut self, i: &'ast syn::ItemStatic) {
        self.add_symbol(i.ident.to_string());
    }

    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        let s = i.ident.to_string();

        if !is_bindgen(&s) {
            self.add_symbol(s);
        }
    }

    fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
        let s = i.ident.to_string();

        match *i.ty {
            syn::Type::BareFn(_) => _ = self.pointer_types.insert(s.clone()),
            syn::Type::Path(ref ty_path) => {
                ty_path.path.segments.last().iter().for_each(|p| {
                    let ty_name = p.ident.to_string();

                    if ty_name == "Option" {
                        if let syn::PathArguments::AngleBracketed(angle_args) = &p.arguments
                            && let Some(syn::GenericArgument::Type(ty)) = angle_args.args.first()
                            && matches!(
                                ty,
                                syn::Type::BareFn(_) | syn::Type::Ptr(_) | syn::Type::Reference(_)
                            )
                        {
                            self.pointer_types.insert(s.clone());
                        }
                    } else if ty_name == "NonNull" {
                        self.pointer_types.insert(s.clone());
                    } else if ty_name.chars().next().is_some_and(|c| c.is_lowercase())
                        || self.primitive_types.contains(&ty_name)
                    {
                        self.primitive_types.insert(s.clone());
                    }
                });
            }
            syn::Type::Ptr(_) => _ = self.pointer_types.insert(s.clone()),
            syn::Type::Reference(_) => _ = self.pointer_types.insert(s.clone()),
            _ => (),
        }
        self.add_symbol(s);
    }

    fn visit_item_union(&mut self, i: &'ast syn::ItemUnion) {
        let s = i.ident.to_string();

        if !is_bindgen(&s) {
            self.add_symbol(s);
        }
    }
}

fn is_bindgen(s: &str) -> bool {
    s.contains("__bindgen_")
}

fn find_places<P: AsRef<Path>, S: AsRef<str>>(path: P, sym: S) -> Places {
    fn search(path: &Path, args: &[&str]) -> Places {
        static COMMON_ARGS: [&str; 23] = [
            "-l",
            "-g",
            "!/.gitlab",
            "-g",
            "!/bindisttest",
            "-g",
            "!/distrib",
            "-g",
            "!/ghc",
            "-g",
            "!/hadrian",
            "-g",
            "!/linters",
            "-g",
            "!/m4",
            "-g",
            "!/mk",
            "-g",
            "!/nofib",
            "-g",
            "!/rust",
            "-g",
            "!/rts",
        ];

        let output = Command::new("rg")
            .current_dir(path)
            .args(COMMON_ARGS)
            .args(args)
            .output()
            .unwrap();

        let mut places = Places::new();

        match std::str::from_utf8(&output.stdout) {
            Ok(files) => {
                for line in files.lines() {
                    if let Some((place, _)) = line.split_once("/") {
                        match place {
                            "compiler" => places.insert(Place::Compiler),
                            "docs" => places.insert(Place::Docs),
                            "libraries" => places.insert(Place::Libraries),
                            "testsuite" => places.insert(Place::Testsuite),
                            "utils" => places.insert(Place::Utils),
                            place => eprintln!("WARN: unexpected place {place}"),
                        };
                    }
                }
            }
            Err(err) => eprintln!("WARN: non-utf8: {err}"),
        }

        places
    }

    let places_c = search(
        path.as_ref(),
        &["-g", "*.{c,h,hsc}", &format!("\\b{}\\b", sym.as_ref())],
    );

    let places_hs = search(
        path.as_ref(),
        &[
            "-g",
            "*.hs",
            &format!("^foreign import .*\\b{}\\b", sym.as_ref()),
        ],
    );

    places_c.union(places_hs)
}

fn print_static_array<T: std::fmt::Debug>(
    ident: &str,
    ty: &str,
    len: usize,
    symbols: impl IntoIterator<Item = T>,
) {
    println!("pub(crate) static {ident}: [{ty}; {len}] = [");

    for sym in symbols {
        println!("    {sym:?},");
    }
    println!("];");
}
