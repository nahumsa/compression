use std::collections::HashMap;

pub fn character_count(input: &str) -> HashMap<char, u32> {
    let mut char_count: HashMap<char, u32> = HashMap::new();
    for char in input.chars() {
        let count = char_count.entry(char).or_insert(0);
        *count += 1;
    }
    char_count
}

#[cfg(test)]
mod tests {
    use super::character_count;
    use std::collections::HashMap;

    #[test]
    fn test_character_count() {
        let input = "hello world";
        let output = HashMap::from([
            ('h', 1),
            ('e', 1),
            ('l', 3),
            (' ', 1),
            ('o', 2),
            ('w', 1),
            ('r', 1),
            ('d', 1),
        ]);
        assert_eq!(character_count(input), output);
    }

    #[test]
    fn test_character_count_only_one_letter() {
        let input = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";
        let output = HashMap::from([('e', 40)]);
        assert_eq!(character_count(input), output);
    }

    #[test]
    fn test_character_count_empty_input() {
        let input = "";
        let output = HashMap::from([]);
        assert_eq!(character_count(input), output);
    }
}
