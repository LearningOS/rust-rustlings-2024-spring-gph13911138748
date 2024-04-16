//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
use std::env;


fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    //env::set_var("TEST_FOO",timestamp.to_string().as_str());
    let your_command = format!(
        "cargo:rustc-env=TEST_FOO={}",
        timestamp
    );
    println!("cargo:{}", your_command);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    //let your_command = "Your command here, please checkout exercises/tests/build.rs";
    //println!("cargo:{}", your_command);
}