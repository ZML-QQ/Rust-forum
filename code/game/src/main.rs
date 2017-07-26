extern crate rand;
extern crate ansi_term;
extern crate postgres;

use std::process::Command;
use std::io::BufRead;
use std::io;
use rand::Rng;
use ansi_term::Colour::Red;
use ansi_term::Colour::Green;
use ansi_term::Colour::Yellow;

mod sql;
mod mode;

struct Data {
    input_line     : String,
    get_letters    : String,
    lives          : i32,
    status         : String,
}
enum UserInput {
    AlreadyExist,
    GuessRight,
    GuessWorng,
}

fn main() {
        let (len,thelines):(usize,Vec<String>) = sql::start();
        let random_line = get_random_line(len,thelines).expect("failed to read input data");
        let mut game: Data =  Data {
                input_line      : random_line,
                get_letters     : String::new(),
                lives           : 5,
                status          : String::new(),
        };
        let mut output_line = format_output_line(&game.input_line,&game.get_letters);
        loop {
            update_screen(&game,&output_line);
            println!("Type your guess letter");
            let user_guess = read_guess();
            if validate_user_guess(user_guess)
            {
                let guess_lower = user_guess.unwrap().to_lowercase().next().unwrap();

                match check_user_guess(&game, guess_lower)
                {
                    UserInput::GuessRight =>
                    {
                        game.get_letters.push(guess_lower);
                        let status = format!("You discovered {}", guess_lower);
                        game.status = Green.paint(status).to_string();
                        output_line = format_output_line(&game.input_line, &game.get_letters);

                        if !output_line.contains('_')
                        {
                            game.status = Green.bold().paint("You won!").to_string();
                            update_screen(&game, &output_line);
                            break;
                        }
                    }

                    UserInput::GuessWorng =>
                    {
                        game.get_letters.push(guess_lower);
                        game.lives = game.lives - 1;

                        if game.lives == 0
                        {
                            game.status = Red.bold().paint("You lost!").to_string();
                            output_line = format_output_line(&game.input_line, &game.input_line);
                            update_screen(&game, &output_line);
                            break;
                        }
                        else
                        {
                            let status = format!("Unfortunately, no {}", guess_lower);
                            game.status = Red.paint(status).to_string();
                        }
                    }

                    UserInput::AlreadyExist =>
                    {
                        let status = format!("{} is already discovered!", guess_lower);
                        game.status = Yellow.paint(status).to_string();
                    }
                }
            }
            else
            {
                let status = format!("It is not a letter!");
                game.status = Yellow.paint(status).to_string();
            }
        }
    }

    fn check_user_guess(game: &Data, user_guess: char) -> UserInput {
        if game.get_letters.contains(user_guess) { return UserInput::AlreadyExist; }
        else if !game.input_line.contains(user_guess) { return UserInput::GuessWorng; }
        UserInput::GuessRight
    }

    fn validate_user_guess(guess: Option<char>) -> bool {
        match guess {
            Some(guess) =>
            {
                if !guess.is_alphabetic() { false }
                else { true }
            }

            None => { return false; }
        }
    }

    fn read_guess() -> Option<char> {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");
        guess.trim().chars().nth(0)
    }

    fn update_screen(game: &Data, output_line: &String) {
        clear();
        println!("welcome: can you guess the sentence ?");
        println!("lives {}, get litters : {}", game.lives, game.get_letters);
        print_result(&game);
        println!("{}", output_line);
        println!("{}", game.status);
    }

    fn print_result(game: &Data) {
        match game.lives
        {
            0 =>
            {
                println!("             ");
                println!(" _________   ");
                println!("|         |  ");
                println!("|         |  ");
                println!("|         |  ");
                println!("|_________|  ");
                println!("             ");
            }

            1 =>
            {
                println!("         ");
                println!("   |     ");
                println!("   |     ");
                println!("   |     ");
                println!("   |     ");
                println!("   |     ");
                println!("          ");
            }

            2 =>
            {
                println!("             ");
                println!("  ________   ");
                println!("         |   ");
                println!("  _______|   ");
                println!(" |           ");
                println!(" |________   ");
                println!("             ");
            }

            3 =>
            {
                println!("             ");
                println!(" _________   ");
                println!("         |   ");
                println!(" ________|   ");
                println!("         |   ");
                println!(" ________|   ");
                println!("             ");

            }

            4 =>
            {
                println!("              ");
                println!(" |      |      ");
                println!(" |      |      ");
                println!(" |______|____  ");
                println!("        |      ");
                println!("        |      ");
                println!("              ");
            }

            _ =>
            {
                println!("             ");
                println!("             ");
                println!("             ");
                println!("             ");
                println!("          O  ");
                println!("         /|\\ ");
                println!("         / \\ ");
            }
        }
    }

    fn clear() {
        let output = Command::new("clear").output().unwrap_or_else(|e| {
             panic!("failed to exex the process{}",e);
        });
        println!("{}",String::from_utf8_lossy(&output.stdout));
    }

    fn format_output_line(input_line: &String,get_letters: &String) -> String {
        let mut result = String::new();
        for (u,c) in input_line.chars().enumerate() {
            result.push(if c == ' ' {c}
            else if get_letters.contains(c) {c}
            else {'_'});
        }
        result
    }

fn get_random_line(len: usize,thelines: Vec<String>) -> Result<String, io::Error > {
    let random_line = rand::thread_rng().gen_range(0,len);
    let input_line = thelines[random_line].clone();
    Ok(input_line)
}
