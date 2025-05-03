use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;
use std::{env, fs};

use build_utils as utils;

fn main() {
    let ghc = utils::GhcDirs::new();

    let bindings_builder = utils::bindgen_builder(&ghc).allowlist_recursively(false);

    let headers_by_dir: HashMap<Option<&str>, (bool, Vec<&str>)> = [
        (None, (true, vec!["Capability"])),
        (
            None,
            (false, vec!["HsFFI", "MachDeps", "Rts", "RtsAPI", "Stg"]),
        ),
        (
            Some("rts"),
            (
                false,
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
        ),
        (Some("rts/prof"), (false, vec!["CCS", "Heap", "LDV"])),
        (
            Some("rts/storage"),
            (
                false,
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
        ),
        (
            Some("stg"),
            (
                false,
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

    let callbacks_state = Rc::new(RefCell::new(CollectCargoCallbacksState {
        filenames: HashSet::new(),
        envs: HashSet::new(),
    }));

    for (path, (internal, headers)) in &headers_by_dir {
        let internal = *internal;

        let (mut include_dir, mut out_dir) = if internal {
            (ghc.root_dir.join("rts"), out_dir.join("rts"))
        } else {
            (ghc.include_dir.clone(), out_dir.clone())
        };

        let (include_dir, out_dir) = match path {
            None => {
                if internal {
                    fs::create_dir_all(&out_dir).unwrap_or_else(|e| {
                        panic!("Unable to create {}: {}", out_dir.display(), e)
                    });
                }
                (include_dir, out_dir)
            }
            Some(path) => {
                let path = PathBuf::from(path);
                out_dir.push(&path);
                fs::create_dir_all(&out_dir)
                    .unwrap_or_else(|e| panic!("Unable to create {}: {}", out_dir.display(), e));
                include_dir.push(path);
                (include_dir, out_dir)
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

                if internal {
                    f.write_all("#include \"Rts.h\"\n".as_bytes()).unwrap();
                } else {
                    if let Some((pre, _)) = rts_h.split_once(&include) {
                        f.write_all(pre.as_bytes()).unwrap();
                    } else if path.is_some() {
                        f.write_all(rts_h.as_bytes()).unwrap();
                    }
                }
                f.write_all(include.as_bytes()).unwrap();
            }
            let bindings = {
                let builder = bindings_builder.clone();

                if internal {
                    builder.clang_arg(format!("-I{}", include_dir.display()))
                } else {
                    builder
                }
                .allowlist_file(header_path.to_string_lossy())
                .header(wrapper_str)
                // Invalidate bindings when header files change.
                .parse_callbacks(Box::new(CollectCargoCallbacks {
                    state: Rc::clone(&callbacks_state),
                }))
                .generate()
                .expect("Unable to generate bindings")
            };
            fs::remove_file(&wrapper).unwrap();

            let module_name = header_to_module(header);

            let module_path = parent_module_path
                .as_ref()
                .map(|s| format!("{}::{}", s, &module_name))
                .unwrap_or(module_name.clone());

            let out_path = out_dir.join(format!("{}.rs", module_name));

            let submodules = fs::read_to_string(header_path)
                .unwrap()
                .lines()
                .filter_map(|line| extract_submodule(&headers_by_dir, &module_path, line))
                .reduce(|s, t| s + t.as_ref());

            let mut out_file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(&out_path)
                .unwrap();

            out_file.write_all(utils::use_libc().as_bytes()).unwrap();

            if let Some(submodules) = submodules {
                out_file.write_all(submodules.as_bytes()).unwrap();
            }

            bindings
                .write(Box::new(out_file))
                .expect("Failed writing bindings");
        }
    }

    callbacks_state.borrow().print_reruns();

    fs::write(&marker, commit).unwrap();
}

#[derive(Debug)]
struct CollectCargoCallbacks {
    state: Rc<RefCell<CollectCargoCallbacksState>>,
}

#[derive(Debug)]
struct CollectCargoCallbacksState {
    filenames: HashSet<String>,
    envs: HashSet<String>,
}

impl CollectCargoCallbacksState {
    fn print_reruns(&self) {
        for filename in self.filenames.iter() {
            println!("cargo:rerun-if-changed={filename}");
        }
        for key in self.envs.iter() {
            println!("cargo:rerun-if-env-changed={key}");
        }
    }
}

impl bindgen::callbacks::ParseCallbacks for CollectCargoCallbacks {
    fn header_file(&self, filename: &str) {
        if !filename.ends_with("wrapper.h") {
            self.state
                .borrow_mut()
                .filenames
                .insert(filename.to_string());
        }
    }

    fn include_file(&self, filename: &str) {
        self.state
            .borrow_mut()
            .filenames
            .insert(filename.to_string());
    }

    fn read_env_var(&self, key: &str) {
        self.state.borrow_mut().envs.insert(key.to_string());
    }
}

fn header_to_module(header: &str) -> String {
    stringcase::snake_case(header)
}

fn extract_submodule(
    headers_by_dir: &HashMap<Option<&str>, (bool, Vec<&str>)>,
    module_path: &str,
    line: &str,
) -> Option<String> {
    if !line.starts_with("#include ") {
        return None;
    }

    let (imp, _) = line.split('"').skip(1).next()?.split_once('.')?;

    let (p, h) = match imp.rsplit_once('/') {
        None => (None, imp),
        Some((p, h)) => (Some(p), h),
    };

    let _ = headers_by_dir.get(&p)?.1.iter().find(|&&s| s == h)?;

    let module_name = header_to_module(h);

    let parent_path = p?;

    let parent_module_path = parent_path.replace('/', "::");

    if parent_module_path == module_path {
        Some(format!("pub mod {};\n", module_name))
    } else {
        None
    }
}
