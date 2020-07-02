use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var_os("NUTTX_DIR");

    if let Some(out_dir) = out_dir {
        let target = Path::new(&out_dir);
        Command::new("make").current_dir(&out_dir).status().unwrap();
        println!("cargo:rustc-link-search={}/staging", target.display());
        println!(
            "cargo:rustc-link-search={}/arch/arm/src/board",
            target.display()
        );
        println!(
            "cargo:rustc-link-search={}/arch/risc-v/src/board",
            target.display()
        );
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
