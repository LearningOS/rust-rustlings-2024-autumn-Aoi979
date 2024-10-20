use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Set up the environment variable TEST_FOO for tests7
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Set the environment variable
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // In tests8, we should enable "pass" feature to make the testcase return early.
    // We need to tell Cargo to link the feature.
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
