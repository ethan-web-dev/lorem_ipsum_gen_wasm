use wasm_bindgen::prelude::*;
use lipsum::lipsum;


// Function to generate Lorem Ipsum text with a given number of words, sentences, or paragraphs
#[wasm_bindgen]
pub fn generate_lorem(count: u32, kind: &str) -> String {
    match kind {
        "words" => lipsum(count as usize),
        "sentences" => lipsum(count as usize * 10),  // Approx 10 words per sentence
        "paragraphs" => lipsum(count as usize * 60), // Approx 60 words per paragraph
        _ => String::from("Invalid type"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_words() {
        let lorem = generate_lorem(5, "words");
        println!("Generated Lorem Ipsum (5 words): {}", lorem);
        assert!(lorem.split_whitespace().count() == 5, "Should generate 5 words");
    }

    #[test]
    fn test_generate_sentences() {
        let lorem = generate_lorem(2, "sentences");
        println!("Generated Lorem Ipsum (2 sentences): {}", lorem);
        assert!(lorem.split_whitespace().count() >= 20, "Should generate around 20 words for 2 sentences");
    }

    #[test]
    fn test_generate_paragraphs() {
        let lorem = generate_lorem(1, "paragraphs");
        println!("Generated Lorem Ipsum (1 paragraph): {}", lorem);
        assert!(lorem.split_whitespace().count() >= 60, "Should generate around 60 words for 1 paragraph");
    }

    #[test]
    fn test_invalid_type() {
        let lorem = generate_lorem(5, "invalid");
        println!("Generated Lorem Ipsum (invalid type): {}", lorem);
        assert_eq!(lorem, "Invalid type", "Should return 'Invalid type' for invalid kind");
    }
}

