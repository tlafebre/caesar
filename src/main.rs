// cope with non letters (.,;<space> etc)
// cope with capitals
// cope with non ascii

use std::collections::HashMap;

const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn caesar(string: &str, shifts: u8) -> Result<String, Box<dyn std::error::Error>> {
    let letters: HashMap<usize, &char> = ALPHABET.iter().enumerate().collect();
    let numbers: HashMap<char, usize> = ALPHABET
        .iter()
        .enumerate()
        .map(|(idx, chr)| (chr.clone(), idx))
        .collect();

    Ok(string
        .chars()
        .map(|c| {
            if c == ' ' {
                c
            } else {
                let shift = numbers[&c] + shifts as usize;
                if shift > 25 {
                    *letters[&(shift % 26)]
                } else {
                    *letters[&shift]
                }
            }
        })
        .collect::<String>())
}

fn main() {
    println!("{:?}", caesar("abcde  fghik", 5).unwrap());

    assert_eq!(caesar("a", 1).unwrap(), String::from("b"));
    assert_eq!(caesar("abcz", 1).unwrap(), String::from("bcda"));
    assert_eq!(caesar("irk", 13).unwrap(), String::from("vex"));
    assert_eq!(caesar("fusion", 6).unwrap(), String::from("layout"));
    assert_eq!(
        caesar("dailyprogrammer", 6).unwrap(),
        String::from("jgorevxumxgsskx")
    );
    assert_eq!(
        caesar("jgorevxumxgsskx", 20).unwrap(),
        String::from("dailyprogrammer")
    );
}
