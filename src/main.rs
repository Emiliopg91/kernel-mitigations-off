use std::{fs, path::PathBuf};

use glob::glob;

#[cfg(debug_assertions)]
const LOADER_ENTRIES_PATH: &str =
    "/var/mnt/Datos/Desarrollo/Workspace/VSCode/kernel-mitigations-off/__test__/*.conf";

#[cfg(not(debug_assertions))]
const LOADER_ENTRIES_PATH: &str = "/boot/loader/entries/*.conf";

fn on_pre() {
    let entries: Vec<PathBuf> = glob(LOADER_ENTRIES_PATH)
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    for entry in entries {
        fs::copy(&entry, format!("{}.bk", &entry.to_string_lossy())).unwrap();
    }
}

fn on_post() {
    let entries: Vec<PathBuf> = glob(LOADER_ENTRIES_PATH)
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    for entry in entries {
        let bk_entry = format!("{}.bk", entry.to_string_lossy());
        println!("  ==> {}", entry.file_stem().unwrap().to_string_lossy());

        if fs::metadata(&entry).unwrap().modified().unwrap()
            <= fs::metadata(&bk_entry).unwrap().modified().unwrap()
        {
            println!("    Up to date");
            continue;
        }

        let lines: Vec<String> = fs::read_to_string(&entry)
            .unwrap()
            .lines()
            .map(|line| {
                let mut line = line.trim().to_string();

                if line.starts_with("options ") {
                    if !line.contains("split_lock_detect") {
                        line = line + " split_lock_detect=off";
                        println!("    Disabling split lock")
                    }
                    if !line.contains("mitigations=off") {
                        line = line + " mitigations=off";
                        println!("    Disabling CPU mitigations")
                    }
                }

                return line;
            })
            .collect();

        fs::write(&entry, lines.join("\n")).unwrap();
        if fs::remove_file(&bk_entry).is_err() {}
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--pre".to_string()) {
        on_pre();
    } else if args.contains(&"--post".to_string()) {
        on_post();
    }
}
