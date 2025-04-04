// build.rs
fn main() {
    // Link against libpython
    println!("cargo:rustc-link-lib=python3.10"); // Match your Python version
    
    // If needed, add library search path
    if let Ok(python_lib) = std::env::var("PYTHON_LIB") {
        println!("cargo:rustc-link-search=native={}", python_lib);
    }
}