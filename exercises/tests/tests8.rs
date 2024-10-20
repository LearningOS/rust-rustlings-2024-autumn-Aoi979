use std::env;

fn main() {
    // Check if the "pass" feature should be enabled
    if let Ok(feature) = env::var("CARGO_FEATURE_PASS") {
        // If the "pass" feature is set, do nothing
        if feature == "1" {
            println!("cargo:rustc-cfg=feature=\"pass\"");
        }
    }
}
