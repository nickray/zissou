// Copyright 2018 by Brandon Edens.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Author: Brandon Edens <brandonedens@gmail.com>
// Date: 2018-11-10

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .flag("-std=c11")
        // .flag("-Os") // doesn't seem to do anything?
        .opt_level_str("s") // maybe implied by release profile?
        .include("/usr/include/newlib")  // might fix the CI build :/
        .define("LFS_NO_MALLOC", None)
        .define("LFS_NO_ASSERT", None)
        .define("LFS_NO_DEBUG", None)
        .define("LFS_NO_WARN", None)
        .define("LFS_NO_ERROR", None)
        .file("littlefs/lfs.c")
        .file("littlefs/lfs_util.c")
        .file("littlefs/string.c")
        .compile("lfs-sys");

    let bindings = bindgen::Builder::default()
        .header("littlefs/lfs.h")
        .use_core()
        .ctypes_prefix("cty")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
