trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    fn append_bar(self) -> Self {
        // Append "Bar" to the original string
        let mut result = self; // Move the original string
        result.push_str("Bar"); // Append "Bar"
        result // Return the modified string
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
