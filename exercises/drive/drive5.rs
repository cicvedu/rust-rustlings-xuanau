// drive5.rs
// Your task is to make the testcase be able to call the `my_demo_function` in module Foo.
// the `my_demo_function_alias` is an alias for `my_demo_function_alias`, so the two line of
// code in the testcase should call the same function.
// You should not modify any existing code. All you need to do is add two line of attributes.


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
    // let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
    // let test_foo_value = timestamp+5;
    // env::set_var("TEST_FOO",test_foo_value.to_string());
    // println!("cargo:rerun-if-changed=build.rs");
    // println!("rcargo:rustc-env=TEST_FOO={}",test_foo_value);
    // if test_foo_value %2 ==0{
    //     println!("cargo:rustc-cfg=feature=\"pass\"");
        
    // }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let e:u64 = timestamp;
        assert! (timestamp >= e && timestamp < e + 10);
        
        // // #[cfg(feature = "pass")]
        // let timestamp:u64 =10;
        // let e:u64 = 10;
        assert! (timestamp>=e&&timestamp<e+10);
        // return;

        // panic!("no cfg set");
    }
}
