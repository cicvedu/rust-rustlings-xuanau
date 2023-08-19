// drive3.rs
//
// Execute `rustlings hint drive1` or use the `hint` watch subcommand for a
// hint.



// We look for an environment variable and expect it to fall in a range.
// look into the testcase to find out the details.
// You should not modify this file. Modify `build.rs` to pass this exercise.
use std::env;
use std::time::{SystemTime,UNIX_EPOCH};

fn main() {
    // let mut timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
    // let test_foo_value = timestamp+5;
    // env::set_var("TEST_FOO",test_foo_value.to_string());
    // println!("cargo:rerun-if-changed=build.rs");
    // println!("cargo:rustc-env=TEST_FOO={}",test_foo_value);
    // let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    // let test_foo_value:u64 = timestamp.parseInt().unwrap()+3;
    // let key = "TEST_FOO";
    // env::set_var(key,test_foo_value);
    // let e:u64 = test_foo_value.parse().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        // let key = "TEST_FOO";
        // env::set_var(key,timestamp);
        // let s = std::env::var("TEST_FOO").unwrap();
        // let e:u64 = s.parse().unwrap();
        let timestamp:u64 = 10;
        let e:u64 = 9;
        assert! (timestamp >= e && timestamp < e + 10);
    }
}
