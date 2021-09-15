use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[cfg(feature = "cuda-10-2")]
const CUDA_VERSION: &str = "10.2";
#[cfg(feature = "cuda-11-1")]
const CUDA_VERSION: &str = "11.1";
#[cfg(feature = "cuda-11-4")]
const CUDA_VERSION: &str = "11.4";

fn main() {
    let cudart = format!("cudart-{}", CUDA_VERSION);
    let nvjpeg = format!("nvjpeg-{}", CUDA_VERSION);
    let cudart_lib = pkg_config::probe_library(&cudart).unwrap();
    let nvjpeg_lib = pkg_config::probe_library(&nvjpeg).unwrap();
    let clang_args: Vec<String> = cudart_lib
        .include_paths
        .iter()
        .chain(nvjpeg_lib.include_paths.iter())
        .map(|x| format!("-I{}", x.display()))
        .collect();

    println!("cargo:args={:?}", clang_args);

    let wrapper_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("wrapper.h");
    let wrapper_path = wrapper_path.to_str().unwrap();
    let mut wrapper = File::create(wrapper_path).unwrap();
    writeln!(wrapper, "#include <cuda_runtime_api.h>").unwrap();
    writeln!(wrapper, "#include <nvjpeg.h>").unwrap();

    let bindings = bindgen::Builder::default()
        .header(wrapper_path)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .anon_fields_prefix("un")
        .derive_debug(true)
        .impl_debug(false)
        .derive_default(true)
        .derive_partialeq(true)
        .derive_eq(true)
        .impl_partialeq(true)
        .allowlist_function("^cuda.*")
        .allowlist_function("^nvjpeg.*")
        .allowlist_var("^NVJPEG.*")
        .clang_args(&clang_args)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
