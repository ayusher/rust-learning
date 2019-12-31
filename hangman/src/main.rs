extern crate rpassword;

use rpassword::read_password;
use std::collections::LinkedList;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::fs;


fn main() {
    let mut parts: LinkedList<String> = LinkedList::new();
    print!("Enter the word to guess: ");
    io::stdout().flush();
    let mut s = read_password().unwrap();
    println!();
    s = s.trim_end().to_string();
    for p in [String::from("a head"), String::from("a neck"), String::from("arms"), String::from("hands"), String::from("legs"), String::from("feet")].iter() {
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
        let mut m = HashMap::new();
        m.insert(String::from("a head"), String::from("src/neck.txt"));
        m.insert(String::from("a neck"), String::from("src/arms.txt"));
        m.insert(String::from("arms"), String::from("src/hands.txt"));
        m.insert(String::from("hands"), String::from("src/legs.txt"));
        m.insert(String::from("legs"), String::from("src/feet.txt"));
        m.insert(String::from("feet"), String::from("src/none.txt"));

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
            if sp[0]=="word:" {
                if sp[1..].connect(" ") == self.word {
                    println!("Congrats, you have guessed the word {:?}", self.word);
                    break;
                } else {
                    println!("Incorrect, you have lost a {}", self.body_parts.pop_back().unwrap());
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
            if self.body_parts.len()>0{
                let mut temp: String = self.body_parts.pop_back().unwrap();
                let contents = &fs::read_to_string(m.get(&temp).unwrap()).unwrap();
                println!("{}", contents);
                self.body_parts.push_back(temp);
            } else {
                let contents = &fs::read_to_string("src/head.txt").unwrap();
                println!("{}", contents);
            }
            
        }
        println!("The word was {:?}!", self.word);
    }
}

  