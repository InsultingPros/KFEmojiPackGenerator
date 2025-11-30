use assert_cmd::cargo;
use kf_emoji_generator::util::get_dir_files;
use serial_test::serial;
use std::{fs, path::Path, process::Command};

const GENERATED_INPUT_PATH: &str = "input";

#[test]
#[serial]
fn copy_files() {
    let reference_input_path: &Path = &Path::new("tests").join("input");
    let target_input: &Path = Path::new(GENERATED_INPUT_PATH);

    if !target_input.exists() {
        std::fs::create_dir(target_input).expect("unable to create input dir!");
    }
    // println!(
    //     "reference_input_path: {reference_input_path:?}, exist: {}, target_input: {target_input:?} (exists: {})",
    //     reference_input_path.exists(),
    //     target_input.exists()
    // );
    if let Ok(reference_files) = get_dir_files(reference_input_path) {
        for file in reference_files {
            if let Some(file_name) = file.file_name() {
                // println!("{file_name:?}");
                if let Err(e) = fs::copy(file.as_path(), target_input.join(file_name)) {
                    eprintln!("Error happened while copying files: {e}");
                }
            }
        }
    } else {
        eprintln!("ops couldn't do anything");
    }
}

#[test]
#[serial]
fn empty_args() {
    assert!(
        Command::new(cargo::cargo_bin!("kf_emoji_generator"))
            .status()
            .expect("failed to execute process")
            .success()
    );
}

#[test]
#[serial]
fn explicit_help_message() {
    assert!(
        Command::new(cargo::cargo_bin!("kf_emoji_generator"))
            .arg("-h")
            .status()
            .expect("failed to execute process")
            .success()
    );
}

#[test]
#[serial]
fn package_name_specified() {
    assert!(
        Command::new(cargo::cargo_bin!("kf_emoji_generator"))
            .arg("--package=test")
            .status()
            .expect("failed to execute process")
            .success()
    );
}

#[test]
#[serial]
fn no_package_name_specified() {
    assert!(
        Command::new(cargo::cargo_bin!("kf_emoji_generator"))
            .status()
            .expect("failed to execute process")
            .success()
    );
}

#[test]
#[serial]
fn custom_size() {
    assert!(
        Command::new(cargo::cargo_bin!("kf_emoji_generator"))
            .arg("--dimensions=48")
            .status()
            .expect("failed to execute process")
            .success()
    );
}

#[test]
#[serial]
fn dxt_mips_masked() {
    assert!(
        Command::new(cargo::cargo_bin!("kf_emoji_generator"))
            .arg("--dxt=99")
            .arg("--mips=99")
            .arg("--masked=99")
            .status()
            .expect("failed to execute process")
            .success()
    );
}
