// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!

#[macro_use]
mod my_mod {
    macro_rules! my_macro {
        ($s:expr) => {{
            let mut a = "Hello ".to_owned();
            a.push_str($s);
            a
        }};
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_macro_world() {
        println!("{}", my_macro!("world!"));
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
}
