use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

use bindgen_utils as utils;

fn main() {
    let ghc = utils::GhcDirs::new();

    let bindings_builder = utils::bindgen_builder(&ghc).allowlist_recursively(false);

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

    // HACK: Avoid rebuilding if GHC hasn't changed.
    let commit = Command::new("git")
        .current_dir(&ghc.root_dir)
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .unwrap()
        .stdout;

    let marker = out_dir.join(".ghc-commit");

    if marker.exists() {
        let last_commit = fs::read(&marker).unwrap();
        if last_commit == commit {
            return;
        }
    }

    let wrapper = out_dir.join("wrapper.h");
    let wrapper_str = wrapper.to_str().unwrap();

    let rts_h = {
        let h = fs::read_to_string(ghc.include_dir.join("Rts.h")).unwrap();
        h.split_once("/* Misc stuff without a home */")
            .unwrap()
            .0
            .to_string()
    };

    for (path, headers) in &headers_by_dir {
        let (include_dir, out_dir) = match path {
            None => (ghc.include_dir.clone(), out_dir.clone()),
            Some(path) => {
                let path = PathBuf::from(path);
                let out_dir = out_dir.join(&path);
                fs::create_dir_all(&out_dir)
                    .unwrap_or_else(|e| panic!("Unable to create {}: {}", out_dir.display(), e));
                (ghc.include_dir.join(path), out_dir)
            }
        };

        let parent_module_path = path.clone().map(|s| s.to_string().replace('/', "::"));

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

            let module_path = parent_module_path
                .as_ref()
                .map(|s| format!("{}::{}", s, &module_name))
                .unwrap_or(module_name.clone());

            let out_path = out_dir.join(format!("{}.rs", module_name));

            let imports_submodules = fs::read_to_string(header_path)
                .unwrap()
                .lines()
                .filter_map(|line| extract_import(&headers_by_dir, &module_path, line))
                .collect::<Vec<_>>();

            let mut out_file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(&out_path)
                .unwrap();

            out_file.write_all(utils::use_libc().as_bytes()).unwrap();

            if !imports_submodules.is_empty() {
                let mut imports = String::new();
                let mut submodules = String::new();

                for is in imports_submodules {
                    match is {
                        ImportOrSubmodule::Import(i) => imports.push_str(&i),
                        ImportOrSubmodule::Submodule(s) => submodules.push_str(&s),
                    }
                }
                if !imports.is_empty() {
                    imports.push_str("\n");
                    out_file.write_all(imports.as_bytes()).unwrap();
                }
                if !submodules.is_empty() {
                    out_file.write_all(submodules.as_bytes()).unwrap();
                }
            }

            bindings
                .write(Box::new(out_file))
                .expect("Failed writing bindings");
        }
    }

    fs::write(&marker, commit).unwrap();
}

fn header_to_module(header: &str) -> String {
    stringcase::snake_case(header)
}

enum ImportOrSubmodule {
    Import(String),
    Submodule(String),
}

fn extract_import(
    headers_by_dir: &HashMap<Option<&str>, Vec<&str>>,
    module_path: &str,
    line: &str,
) -> Option<ImportOrSubmodule> {
    if !line.starts_with("#include ") {
        return None;
    }

    let (imp, _) = line.split('"').skip(1).next()?.split_once('.')?;

    let (p, h) = match imp.rsplit_once('/') {
        None => (None, imp),
        Some((p, h)) => (Some(p), h),
    };

    let _ = headers_by_dir.get(&p)?.iter().find(|&&s| s == h)?;

    let module_name = header_to_module(h);

    match p {
        None => Some(ImportOrSubmodule::Import(format!(
            "use crate::{};\n",
            &module_name
        ))),
        Some(p) => {
            let parent_module_path = p.replace('/', "::");

            if parent_module_path == module_path {
                Some(ImportOrSubmodule::Submodule(format!(
                    "pub mod {};\n",
                    module_name
                )))
            } else {
                Some(ImportOrSubmodule::Import(format!(
                    "use crate::{}::{};\n",
                    parent_module_path, module_name
                )))
            }
        }
    }
}
