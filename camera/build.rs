fn main() {
    // Set the linker flags
    println!("cargo:rustc-link-lib=rscamera");
    println!("cargo:rustc-link-lib=camera");
    println!("cargo:rustc-link-lib=camera-base");

    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-search=./lib");
}
