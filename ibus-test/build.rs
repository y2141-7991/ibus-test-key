extern crate cc;
extern crate pkg_config;

fn main() {
    println!("cargo:rustc-link-lib=ibus-1.0");
    let libs = pkg_config::Config::new()
        .atleast_version("1.0")
        .probe("ibus-1.0")
        .expect("Failed to find ibus-1.0 using pkg-config");

    let glib_libs = pkg_config::Config::new()
        .atleast_version("2.0")
        .probe("glib-2.0")
        .expect("Failed to find glib-2.0 using pkg-config");

    cc::Build::new()
        .file("wrapper.c")
        .includes(&libs.include_paths)
        .includes(&glib_libs.include_paths)
        .warnings(true)
        .compile("wrapper");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=wrapper.c");

    for path in libs.link_paths {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    for lib in libs.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }

    for path in glib_libs.link_paths {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    for lib in glib_libs.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }
}
