use unicode_segmentation::UnicodeSegmentation;

// Write a function that reverses a string
// For example, input of "robot" should return "tobor"
// The function should also work on Unicode strings using the UnicodeSegmentation library, reversing them correctly
pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}
