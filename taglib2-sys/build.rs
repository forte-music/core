fn main() {
    let dst = cmake::build("taglib");

    cc::Build::new()
        .cpp(true)
        .file("src/wrapper.cpp")
        .include("taglib/taglib/toolkit")
        .include("taglib/taglib")
        .compile("wrapper");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=tag");
    println!("cargo:rustc-link-lib=z");
}
