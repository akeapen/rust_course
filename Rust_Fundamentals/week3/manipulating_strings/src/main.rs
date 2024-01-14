#[derive(Debug)]
struct Vowels {
    a: u32,
    e: u32,
    i: u32,
    o: u32,
    u: u32,
}

fn update_vowels(v: &mut Vowels, c: char) {
    match c {
        'a' => v.a += 1,
        'e' => v.e += 1,
        'i' => v.i += 1,
        'o' => v.o += 1,
        'u' => v.u += 1,
        _ => (),
    }
}

fn get_longest_word(sentence: &str) -> &str {
    let mut longest = "";
    for word in sentence.split_whitespace() {
        if word.len() >= longest.len() {
            longest = word;
        }
    }
    longest
}

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    //println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    println!("{}", description);

    let v = &mut Vowels {a: 0, e: 0, i: 0, o: 0, u: 0};

    // iterate over the characters in the sentence
    for c in sentence.chars() {
        update_vowels(v, c);
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
            _ => continue,
        }
    }
    println!("{:#?}", v );

    // Split and collect into a vector
    //let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    println!("{:?}", words);
    let longest_word = get_longest_word(&sentence);
    println!("longest word is: {}", longest_word);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);
}
