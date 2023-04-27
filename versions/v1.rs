use std::io;

fn avaiable_word(word:&String,letters:&[u8]) -> bool{
    let mut letters = Vec::from(letters);
    for w in word.bytes(){
        if letters.contains(&w){
            letters.remove(letters.iter().position(|&r| r == w).unwrap());
        }else{
            return false;
        }
    }
    return true;
}

fn word_value(word:&String) -> i32{

    let value:i32 = word.chars().fold(0, |v,c| v+(match c{
        'e'| 'a'| 'i'| 'o'| 'n'| 'r'| 't'| 'l'| 's'| 'u' => 1,
        'd' | 'g' => 2,
        'b' | 'c'|'m'|'p' => 3,
        'f' |  'h' |  'v' |  'w' |  'y' => 4,
        'k' => 5,
        'j' | 'x' => 8,
        'q'  | 'z' => 10,
        _ => 0,
    }));

    return value;
}

fn main() {
    let mut words_n = String::new();
    io::stdin().read_line(&mut words_n).unwrap();

    let words_n:isize = words_n.trim().parse().unwrap();
    let mut words = Vec::new();

    for _ in 0..words_n{
        let mut word = String::new();
        io::stdin().read_line(&mut word).unwrap();
        words.push(word.trim().to_string());
    }
    let mut letters:String = String::new();
    io::stdin().read_line(&mut letters).unwrap();
    let letters = letters.as_bytes();
    let mut words:Vec<String> = words.into_iter().filter(|word| avaiable_word(word,letters)).collect();
    eprintln!("{:?}", words);
    let max_value = words.iter().fold(0, |m,s| m.max(word_value(s)));
    eprintln!("{}",max_value);
    let winner_word = words.iter().find(|w| word_value(w) == max_value);
    println!("{}", winner_word.unwrap());
}
