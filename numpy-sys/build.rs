
extern crate glob;

use std::env::var;
use std::fs::canonicalize;
use std::path::PathBuf;
use std::process::Command;
use glob::glob;

fn glob1(regex: &PathBuf) -> PathBuf {
    glob(regex.to_str().unwrap()).unwrap().last().unwrap().unwrap()
}

fn search_lib(libpath: &PathBuf, regex: &str) -> PathBuf {
    glob1(&libpath.join(regex))
}

fn create_symlink(libpath: &PathBuf, lib: &PathBuf, name: &str) {
    Command::new("ln")
        .arg("-sf")
        .arg(lib.file_name().unwrap())
        .arg(name)
        .current_dir(&libpath)
        .status()
        .expect("Failed to create symlink");
}

fn main() {
    let output = PathBuf::from(var("OUT_DIR").unwrap().replace(r"\", "/"));
    let source = PathBuf::from("numpy");
    Command::new("python3")
        .arg("setup.py")
        .arg("build")
        .arg("--build-base")
        .arg(&output)
        .current_dir(&source)
        .status()
        .expect("Failed to build numpy");
    let libpath = canonicalize(glob1(&output.join("lib.*/numpy/core"))).unwrap();
    let multiarray = search_lib(&libpath, "multiarray.*.so");
    let umath = search_lib(&libpath, "umath.*.so");
    create_symlink(&libpath, &multiarray, "libmultiarray.so");
    create_symlink(&libpath, &umath, "libumath.so");
    println!("cargo:rustc-link-search={}", libpath.display());
    println!("cargo:rustc-link-lib=dylib=multiarray");
    println!("cargo:rustc-link-lib=dylib=umath");
}
