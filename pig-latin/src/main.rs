fn main() {
    let new_sentence = pig_latin(&String::from("i love horses best of all the animals"));
    println!("{}", new_sentence);
}

// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn pig_latin(sentence: &str) -> String {
    let words = sentence.split_whitespace();
    let mut new_phrase = String::from("");
    for word in words.into_iter() {
        let new_word = match &word[0..1] {
            "a" | "e" | "i" | "o" | "u" => format!("{}-ay", word),
            _ => format!("{}-{}ay", &word[1..], &word[0..1])
        };
        new_phrase = new_phrase + " " + &new_word;
    }
    new_phrase.trim().to_string()
}