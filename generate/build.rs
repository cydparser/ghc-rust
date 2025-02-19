use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let ghc_dir = std::env::var("GHC_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| manifest_dir.parent().unwrap().parent().unwrap().to_owned());

    let build_dir = ghc_dir.join(PathBuf::from("_build/stage1/rts/build"));

    let include_dir = ghc_dir.join(PathBuf::from("rts/include"));

    let bindings_builder = bindgen::Builder::default()
        .allowlist_recursively(false)
        .clang_arg(format!("-I{}", include_dir.display()))
        .clang_arg(format!("-I{}", build_dir.join("include").display()))
        // Invalidate bindings when header files change.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    let headers_by_dir: HashMap<Option<&str>, Vec<&str>> = [
        (None, vec!["HsFFI", "MachDeps", "Rts", "RtsAPI", "Stg"]),
        (
            Some("rts"),
            vec![
                "Adjustor",
                "BlockSignals",
                "Config",
                "Constants",
                "EventLogWriter",
                "ExecPage",
                "FileLock",
                "Flags",
                "ForeignExports",
                "GetTime",
                "Globals",
                "Hpc",
                "IOInterface",
                "IPE",
                "Libdw",
                "LibdwPool",
                "Linker",
                "Main",
                "Messages",
                "NonMoving",
                "OSThreads",
                "Parallel",
                "PrimFloat",
                "Profiling",
                "Signals",
                "SpinLock",
                "StableName",
                "StablePtr",
                "StaticPtrTable",
                "TSANUtils",
                "TTY",
                "Threads",
                "Ticky",
                "Time",
                "Timer",
                "Types",
                "Utils",
            ],
        ),
        (Some("rts/prof"), vec!["CCS", "Heap", "LDV"]),
        (
            Some("rts/storage"),
            vec![
                "Block",
                "ClosureMacros",
                "ClosureTypes",
                "Closures",
                "FunTypes",
                "GC",
                "Heap",
                "InfoTables",
                "MBlock",
                "TSO",
            ],
        ),
        (
            Some("stg"),
            vec![
                "DLL",
                "MachRegsForHost",
                "MiscClosures",
                "Prim",
                "Regs",
                "SMP",
                "Ticky",
                "Types",
            ],
        ),
    ]
    .into_iter()
    .collect();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let wrapper = manifest_dir.join("wrapper.h");
    let wrapper_str = wrapper.to_str().unwrap();

    let rts_h = fs::read_to_string(include_dir.join("Rts.h")).unwrap();

    for (path, headers) in &headers_by_dir {
        let (include_dir, out_dir) = match path {
            None => (include_dir.clone(), out_dir.clone()),
            Some(path) => {
                let path = PathBuf::from(path);
                let out_dir = out_dir.join(&path);
                fs::create_dir_all(&out_dir)
                    .unwrap_or_else(|e| panic!("Unable to create {}: {}", out_dir.display(), e));
                (include_dir.join(path), out_dir)
            }
        };

        for header in headers {
            let header_path = include_dir.join(format!("{}.h", header));

            {
                let mut f = fs::File::create(&wrapper).unwrap();

                let include = format!(
                    "#include \"{}.h\"",
                    match path {
                        None => header.to_string(),
                        Some(path) => format!("{}/{}", path, header),
                    }
                );

                if let Some((pre, _)) = rts_h.split_once(&include) {
                    f.write_all(pre.as_bytes()).unwrap();
                } else if path.is_some() {
                    f.write_all(rts_h.as_bytes()).unwrap();
                }
                dbg!(&include);
                f.write_all(include.as_bytes()).unwrap();
            }
            let bindings = bindings_builder
                .clone()
                .allowlist_file(header_path.to_string_lossy())
                .header(wrapper_str)
                .generate()
                .expect("Unable to generate bindings");

            fs::remove_file(&wrapper).unwrap();

            let module_name = header_to_module(header);

            let out_path = out_dir.join(format!("{}.rs", module_name));

            let imports = fs::read_to_string(header_path)
                .unwrap()
                .lines()
                .filter_map(|line| extract_import(&headers_by_dir, line))
                .collect::<Vec<_>>();

            if !imports.is_empty() {
                dbg!((path, header, &imports));
                fs::write(&out_path, imports.concat()).unwrap();
            }

            bindings
                .write_to_file(out_path)
                .expect("Failed writing bindings");
        }
    }
}

fn header_to_module(header: &str) -> String {
    stringcase::snake_case(header)
}

fn extract_import(headers_by_dir: &HashMap<Option<&str>, Vec<&str>>, line: &str) -> Option<String> {
    if !line.starts_with("#import ") {
        return None;
    }

    let (imp, _) = line.split('"').skip(1).next()?.split_once('.')?;

    let (p, h) = match imp.rsplit_once('/') {
        None => (None, imp),
        Some((p, h)) => (Some(p), h),
    };

    let _ = headers_by_dir.get(&p)?.iter().find(|&&s| s == h)?;

    Some(format!(
        "use crate::{}{};\n",
        p.map(|s| format!("{}::", s.replace('/', "::")))
            .unwrap_or(String::new()),
        header_to_module(h)
    ))
}
