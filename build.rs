use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    if !target.starts_with("x86") {
        let nuttx_dir = env::var_os("NUTTX_DIR");
        if let Some(nuttx_dir) = nuttx_dir {
            let nuttx = Path::new(&nuttx_dir);

            if target.starts_with("thumb") {
                println!(
                    "cargo:rustc-link-search={}/arch/arm/src/board",
                    nuttx.display()
                );
            } else if target.starts_with("riscv") {
                println!(
                    "cargo:rustc-link-search={}/arch/risc-v/src/board",
                    nuttx.display()
                );
            } else {
                println!("cargo:warning=Platform not support:{}", target);
            }

            Command::new("make").current_dir(&nuttx).status().unwrap();

            println!("cargo:rustc-link-search={}/staging", nuttx.display());
            println!("cargo:rustc-link-lib=static=sched");
            println!("cargo:rustc-link-lib=static=drivers");
            println!("cargo:rustc-link-lib=static=boards");
            println!("cargo:rustc-link-lib=static=arch");
            println!("cargo:rustc-link-lib=static=c");
            println!("cargo:rustc-link-lib=static=mm");
            println!("cargo:rustc-link-lib=static=xx");
            println!("cargo:rustc-link-lib=static=apps");
            println!("cargo:rustc-link-lib=static=fs");
            println!("cargo:rustc-link-lib=static=binfmt");
            println!("cargo:rustc-link-lib=static=board");
        } else {
            println!("cargo:warning=No NuttX source dir found!");
        }
    }
}
