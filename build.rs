extern crate num_cpus;

use std;
use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();

    if target.starts_with("x86") {
        return;
    }

    let nuttx_src_dir = env::var("NUTTX_SRC_DIR");
    let nuttx_board_dir = env::var("NUTTX_BOARD_DIR");
    let nuttx_board_ld = env::var("NUTTX_BOARD_LD");

    if let Ok(nuttx_src_dir) = nuttx_src_dir {
        let nuttx_path = Path::new(&nuttx_src_dir);
        let mut board_path: String = "boards/arm/stm32/stm32f4discovery".to_string();
        let mut board_ld: String = "ld.script".to_string();
        if target.starts_with("thumb") {
            println!(
                "cargo:rustc-link-search={}/arch/arm/src/board",
                nuttx_path.display()
            );
        } else if target.starts_with("riscv") {
            println!(
                "cargo:rustc-link-search={}/arch/risc-v/src/board",
                nuttx_path.display()
            );
        } else {
            println!("cargo:warning=Platform not supported:{}", target);
        }

        if let Ok(dir) = nuttx_board_dir {
            board_path = dir;
        }

        if let Ok(ld) = nuttx_board_ld {
            board_ld = ld;
        }

        distclean(nuttx_path);
        patch(nuttx_path);
        configure(nuttx_path, board_path, board_ld);
        build(nuttx_path);

        println!("cargo:rustc-link-search={}/staging", nuttx_path.display());
        println!("cargo:rustc-link-lib=static=arch");
        println!("cargo:rustc-link-lib=static=apps");
        println!("cargo:rustc-link-lib=static=binfmt");
        println!("cargo:rustc-link-lib=static=boards");
        println!("cargo:rustc-link-lib=static=c");
        println!("cargo:rustc-link-lib=static=drivers");
        println!("cargo:rustc-link-lib=static=fs");
        println!("cargo:rustc-link-lib=static=mm");
        println!("cargo:rustc-link-lib=static=sched");
        println!("cargo:rustc-link-lib=static=board");
    } else {
        println!("cargo:warning=No NuttX source directory found!");
    }
    println!("cargo:rerun-if-changed=build.rs");
}

fn distclean(nx_path: &Path) {
    Command::new("make")
        .arg("distclean")
        .current_dir(nx_path)
        .status()
        .unwrap();
}

fn configure(nx_path: &Path, boards: String, ld: String) {
    Command::new("sh")
        .arg("-c")
        .arg(format!("./tools/configure.sh {}/configs/nsh", boards))
        .current_dir(nx_path)
        .status()
        .unwrap();

    Command::new("cp")
        .arg(format!("{}/{}/scripts/{}", nx_path.display(), boards, ld))
        .arg(format!("link.ld"))
        .status()
        .unwrap();
}

fn patch(nx_path: &Path) {
    Command::new("sh")
        .arg("-c")
        .arg("find . -name defconfig | xargs sed -i 's/nsh_main/main/g'")
        .current_dir(nx_path)
        .status()
        .unwrap();
}

fn build(nx_path: &Path) {
    let jobs = format!("-j{}", num_cpus::get());

    Command::new("make")
        .arg(&jobs)
        .current_dir(nx_path)
        .status()
        .unwrap();
}
