use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G
}

fn main() {
    let mut hash: HashMap<Letter, u32> = HashMap::new();

    hash.insert(Letter::A, 0);
    hash.insert(Letter::B, 0);
    hash.insert(Letter::C, 0);
    hash.insert(Letter::D, 0);

    add_letters(&mut hash);

    println!("{:?}", hash);
}

fn add_letters(hash: &mut HashMap<Letter, u32>) {
    let letters = vec![
        Letter::A,
        Letter::B,
        Letter::C,
        Letter::D,
        Letter::E,
        Letter::F,
        Letter::G
    ];

    for letter in letters {
        if hash.contains_key(&letter) {
            continue
        } else {
            hash.insert(letter, 1);
        }
    }
}