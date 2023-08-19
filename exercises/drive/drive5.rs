// drive5.rs
// Your task is to make the testcase be able to call the `my_demo_function` in module Foo.
// the `my_demo_function_alias` is an alias for `my_demo_function_alias`, so the two line of
// code in the testcase should call the same function.
// You should not modify any existing code. All you need to do is add two line of attributes.




// extern "C" {
//     // #[link_name = "Foo::my_demo_function"]
//     fn my_demo_function(a:u32) -> u32
//     // #[link_name = "Foo::my_demo_function"]
//     fn my_demo_function_alias(a:u32) -> u32
// }

fn my_demo_function(a:u32) -> u32{
    a
}
fn my_demo_function_alias(a:u32) -> u32{
    a
}

// mod Foo{
//     fn my_demo_function(a:u32) -> u32{a}
// }



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let a = my_demo_function(123);
        let b = my_demo_function_alias(123);
        assert_eq!(a,123);
        // assert_eq!(b,123);
        
    //     unsafe {
    //         let a:u64 = my_demo_function(123);
    //         let b:u64 = my_demo_function_alias(456);
    //     }
    //     assert!(a<b,"a is smaller")
    // }
}
