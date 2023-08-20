fn main() {
    println!("cargo:info=Running custom build script!");
    println!("cargo:warning=This is a warning message.");
    println!("cargo:error=This is an error message.");
}