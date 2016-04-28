// Copyright 2016, NICTA
//
// This software may be distributed and modified according to the terms of
// the BSD 2-Clause license. Note that NO WARRANTY is provided.
// See "LICENSE_BSD2.txt" for details.
//
// @TAG(NICTA_BSD)
//

use std::path::Path;
use std::process::Command;
use std::fs;



pub fn build_compiler_rt<P: AsRef<Path>>(rust_dir: P, out_dir: P, llvm_triple: &String) {
    let triple_path = format!{"{}/rt/libcompiler-rt.a", llvm_triple};
    let configure = &mut Command::new("./configure");
    configure.current_dir(rust_dir.as_ref());
    configure.args(&[format!("--target={}", llvm_triple)]);

    assert!(configure.status().expect("Failed to execute command").success());

    let make = &mut Command::new("make");
    make.current_dir(rust_dir.as_ref());
    make.args(&[&triple_path]);

    println!("{:?}", make);
    assert!(make.status().expect("Failed to execute command").success());
    let res = fs::hard_link(rust_dir.as_ref().join(&triple_path),
                            out_dir.as_ref().join("libcompiler-rt.a"));
    res.or_else(|_| {
           fs::copy(rust_dir.as_ref().join(&triple_path),
                    out_dir.as_ref().join("libcompiler-rt.a"))
               .map(|_| ())
       })
       .expect("Failed to link");
}
