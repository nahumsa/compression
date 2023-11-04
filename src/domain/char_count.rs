use std::collections::HashMap;

pub fn character_count(input: &str) -> HashMap<char, u32> {
    let output = HashMap::new();
    output
}

#[cfg(test)]
mod tests {
    use super::character_count;
    use std::collections::HashMap;
    #[test]
    fn test_character_count() {
        let input = "hello world";
        let mut output = HashMap::new();
        output.insert('h', 1);
        output.insert('e', 1);
        output.insert('l', 3);
        output.insert('o', 2);
        output.insert('w', 1);
        output.insert('r', 1);
        output.insert('d', 1);
        assert_eq!(character_count(input), output);
    }
}
