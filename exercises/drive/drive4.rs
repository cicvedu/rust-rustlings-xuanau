// drive4.rs
//
// Execute `rustlings hint drive1` or use the `hint` watch subcommand for a
// hint.


// This execrise shares build.rs with the previous exercise.
// You need to add some code to build.rs to make both this exercise and
// the previous one work.
use std::env;
use std::time::{SystemTime,UNIX_EPOCH};


fn main() {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
    let test_foo_value = timestamp+5;
    env::set_var("TEST_FOO",test_foo_value.to_string());
    println!("cargo:rerun-if-changed=build.rs");
    println!("rcargo:rustc-env=TEST_FOO={}",test_foo_value);
    if test_foo_value %2 ==0{
        println!("cargo:rustc-cfg=feature=\"pass\"");
        
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return;

        panic!("no cfg set");
    }
}
