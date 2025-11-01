use cc::Build;
fn main() {
    Build::new()
        .file("c_code/viennarna.c")
        .include("/opt/homebrew/include")
        .compile("libviennarna");
    
    // for manual compilation with c_code/build.sh:
    // println!("cargo:rustc-link-search=all=c_code");
    // println!("cargo:rustc-link-lib=dylib=viennarna");
    
    println!("cargo:rustc-link-lib=dylib=RNA");
    println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
}
