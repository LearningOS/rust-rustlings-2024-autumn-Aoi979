extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // Add attributes to the functions
    #[no_mangle]
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }

    #[no_mangle]
    #[link_name = "my_demo_function"]
    pub fn my_demo_function_alias(a: u32) -> u32 {
        my_demo_function(a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        unsafe {
            assert_eq!(my_demo_function(123), 123);
            assert_eq!(my_demo_function_alias(456), 456);
        }
    }
}
