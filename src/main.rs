use std::{fs, path::PathBuf};

use glob::glob;

#[cfg(debug_assertions)]
const LOADER_ENTRIES_PATH: &str =
    "/var/mnt/Datos/Desarrollo/Workspace/VSCode/kernel-mitigations-off/__test__/*.conf";

#[cfg(not(debug_assertions))]
const LOADER_ENTRIES_PATH: &str = "/boot/loader/entries/*.conf";

fn on_post() {
    let entries: Vec<PathBuf> = glob(LOADER_ENTRIES_PATH)
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    for entry in entries {
        println!(" ==> {}", &entry.file_stem().unwrap().to_string_lossy());
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
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--post".to_string()) {
        on_post();
    }
}
