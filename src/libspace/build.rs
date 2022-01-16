mod gen_sources;
use gen_sources::*;

use sha2::{Digest, Sha256};
use std::io::Read;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use tar::Archive;

const DOWNLOAD_SPICE_NAME_Z: &'static str = "cspice.tar.Z";
const DOWNLOAD_SPICE_NAME_TAR: &'static str = "cspice.tar";
const DOWNLOAD_CSPICE_NAME: &'static str = "cspice";
const CSPICE_BUILD_SCRIPT_NAME: &'static str = "./makeall.csh";
const DOWNLOAD_SPICE_URL: &'static str =
    "https://naif.jpl.nasa.gov/pub/naif/toolkit//C/PC_Linux_GCC_64bit/packages/cspice.tar.Z";
const DOWNLOAD_SPICE_SHA256: &'static str =
    "60a95b51a6472f1afe7e40d77ebdee43c12bb5b8823676ccc74692ddfede06ce";

fn main() {
    build_cspice();
    gen_kepler();
}

fn download(url: &str, checksum: &str, target_filename: &PathBuf) -> Result<(), String> {
    if target_filename.exists() {
        return Ok(());
    }

    println!("Downloading {}", url);

    let response = ureq::get(url).call().map_err(|error| error.to_string())?;

    if response.status() < 200 || response.status() >= 300 {
        return Err(format!("HTTP Error for {}: {}", url, response.status()));
    }

    let len = response
        .header("Content-Length")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap();
    let mut data: Vec<u8> = Vec::with_capacity(len);
    response.into_reader().read_to_end(&mut data).unwrap();

    let hash = Sha256::digest(&data);
    if &format!("{:x}", hash) != checksum {
        return Err(format!("Checksum for {} failed: ", url));
    }

    std::fs::write(Path::new(target_filename), data).unwrap();

    Ok(())
}
fn download_cspice() {
    let z_name = get_out_dir().join(Path::new(DOWNLOAD_SPICE_NAME_Z));
    download(DOWNLOAD_SPICE_URL, DOWNLOAD_SPICE_SHA256, &z_name)
        .unwrap_or_else(|error| panic!("CSPICE Download failed: {}", error));
}

fn build_cspice() {
    download_cspice();
    let name_tar = get_out_dir().join(DOWNLOAD_SPICE_NAME_TAR);
    let name_cspice_dir = get_out_dir().join(DOWNLOAD_CSPICE_NAME);
    if !name_tar.exists() {
        Command::new("uncompress")
            .args(["-k", DOWNLOAD_SPICE_NAME_Z])
            .current_dir(get_out_dir())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        let mut ar = Archive::new(File::open(name_tar).unwrap());
        ar.unpack(get_out_dir()).unwrap();
    }

    let lib_name = name_cspice_dir.join("lib").join("cspice.a");
    let sup_name = name_cspice_dir.join("lib").join("csupport.a");
    if !lib_name.exists() || !sup_name.exists() {
        Command::new("/bin/sh")
            .args([CSPICE_BUILD_SCRIPT_NAME])
            .current_dir(name_cspice_dir.clone())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    let good_lib_name = name_cspice_dir.join("lib").join("libcspice.a");
    let good_sup_name = name_cspice_dir.join("lib").join("libcsupport.a");
    if !good_lib_name.exists() || !good_sup_name.exists() {
        std::fs::copy(lib_name, good_lib_name).unwrap();
        std::fs::copy(sup_name, good_sup_name).unwrap();
    }

    print!(
        "cargo:rustc-link-search=[{}]",
        name_cspice_dir.join("lib").to_str().unwrap()
    );
    print!("cargo:rustc-link-lib=[cspice]");
    print!(
        "cargo:rustc-cdylib-link-arg=[-I{}]",
        name_cspice_dir.join("lib").to_str().unwrap()
    );
}
