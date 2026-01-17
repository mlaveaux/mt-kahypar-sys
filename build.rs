use std::env;
use std::path::PathBuf;

use bindgen::{Builder, EnumVariation, RustEdition, RustTarget};

fn main() {
    let link_type;
    
    if cfg!(feature = "bundled") {
        link_type = "static";

        // Use the `cmake` crate to build mt-kahypar C++ library.
        let mut dst = cmake::Config::new("mt-kahypar")
            .define("BUILD_SHARED_LIBS", "OFF") // Build a static library.
            .define("KAHYPAR_STATIC_LINK_DEPENDENCIES ", "ON")
            .define("KAHYPAR_DOWNLOAD_TBB", "ON") // Use built-in TBB.
            .define("KAHYPAR_DOWNLOAD_BOOST", "ON") // Use built-in Boost.
            .define("KAHYPAR_STATIC_LINK_TBB ", "ON") // Statically link TBB.
            .define("KAHYPAR_DISABLE_HWLOC", "ON") // Disable hwloc to avoid dependency.
            .build();
        dst.push("build/lib");

        cargo_emit::rustc_link_search!(dst.display() => "native");
        
        // Generate bindings for the C++ interface using bindgen.
        let bindings = Builder::default()
            // The input header we would like to generate
            // bindings for.
            .header("mt-kahypar/include/mtkahypar.h")
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            // Sets various defaults for bindgen.
            .default_enum_style(EnumVariation::Rust { non_exhaustive: false })
            .rust_target(RustTarget::stable(73, 0).expect("Unsupported Rust version"))
            .rust_edition(RustEdition::Edition2021)
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");
        
        // Write the bindings to the $OUT_DIR/bindings.rs file.
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
    else
    {
        link_type = "dylib";

        // When using an already built version, build all possible paths to look for ...
        let dir = env::var("MTKAHYPAR_DIR").map(PathBuf::from).ok();

        let include_dir = dir.clone().map(|dir| dir.join("include"));
        let lib_dir = dir.clone().map(|dir| dir.join("lib"));
        let lib64_dir = dir.map(|dir| dir.join("lib64"));

        // ... and include them for the library search.
        if let Some(dir) = include_dir {
            println!("cargo:include-dir={}", dir.display());
        }

        if let Some(dir) = lib64_dir {
            println!("cargo:rustc-link-search={}", dir.display());
        }

        if let Some(dir) = lib_dir {
            println!("cargo:rustc-link-search={}", dir.display());
        }

        println!("rerun-if-env-changed=MTKAHYPAR_DIR");
    }   

    // Finally, link the library.
    cargo_emit::rustc_link_lib!("mtkahypar" => link_type);
}
