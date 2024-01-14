// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

fn trim_me(input: &str) -> String {
    // shit way
    // let chars_to_trim = &[' '];
    // let binding = input.to_string();
    // let ret = binding.trim_matches(chars_to_trim).to_string();
    // ret

    // good way
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // shit way
    // let mut ret = input.to_string();
    // ret.push_str(" world!");
    // ret

    // good way
    format!("{} world!", input)
}

fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
