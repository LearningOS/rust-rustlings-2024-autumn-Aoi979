use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Get the current timestamp in seconds
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // Set the environment variable TEST_FOO to the current timestamp
    println!("cargo:rerun-if-env-changed=TEST_FOO");
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
}
