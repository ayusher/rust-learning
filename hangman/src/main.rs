use std::collections::LinkedList;
use std::io;
use std::io::Write;

fn main() {
    let mut parts: LinkedList<String> = LinkedList::new();
    print!("Enter the word to guess: ");
    io::stdout().flush();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("bad");
    s = s.trim_end().to_string();
    for p in [String::from("a head"), String::from("a neck"), String::from("arms"), String::from("legs"), String::from("feet")].iter() {
        parts.push_back(p.to_string());
    }
    let mut hang = Hangman {word: s, body_parts: parts};
    hang.play()
}

struct Hangman {
    word: String,
    body_parts: LinkedList<String>,
}

impl Hangman { 
    fn play(&mut self) {
        let mut guesses = Vec::new();
        loop {
            let mut guess = String::new();
            let mut x = "".to_string();
            for c in self.word.chars() {
                if guesses.contains(&c){
                    x.push_str(&format!("{}", c));
                } else{
                    x.push_str(&"_");
                }
            }
            println!("{}", x);
            if self.body_parts.len()==0 {
                println!("Game over, better luck next time!");
                break;
            } else if x==self.word{
                println!("Good job, you've won!");
                break;
            }
            print!("Guess a letter: ");
            io::stdout().flush();
            io::stdin().read_line(&mut guess).expect("bad");
            guess = guess.trim_end().to_string();
            let mut sp: Vec<&str> = guess.split_whitespace().collect();
            if sp.contains(&"word:") {
                if sp[1] == self.word {
                    println!("Congrats, you have guessed the word {:?}", self.word);
                    break;
                } else {
                    println!("Incorrect, you have lost a {:?}", self.body_parts.pop_back());
                }
            } else if guess.chars().count() > 1 || guess.chars().count() < 1 {
                println!("{}", guess);
                println!("Invalid guess, if you would like to guess the entire word type 'word: ' in front of your guess!")
            } else {
                guesses.push(guess.chars().next().unwrap());
                if self.word.contains(&format!("{}", guess)){
                    println!("Well done, there are some {}'s", guess);
                } else {
                    println!("Incorrect, the man has lost {}", self.body_parts.pop_back().unwrap());
                }
            }
        }
        println!("The word was {:?}!", self.word);
    }
}

  