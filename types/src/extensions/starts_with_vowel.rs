pub trait StartsWithVowel {
    fn starts_with_vowel(&self) -> bool;
}

impl StartsWithVowel for str {
    fn starts_with_vowel(&self) -> bool {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        vowels.contains(&self.chars().next().unwrap())
    }
}

impl StartsWithVowel for String {
    fn starts_with_vowel(&self) -> bool {
        self.as_str().starts_with_vowel()
    }
}
