use std::env;
use std::fs;
use std::path::Path;

extern crate colored;
use colored::*;

fn main() {
    let filename: String = if let Some(filename) = env::args().nth(1) {
        filename
    }else{
        panic!("please specify filename with arguments")
    };
    let history_filename: String = {
        let filename = filename.clone();
        let p = Path::new(&filename);
        if let Some(parent) = p.parent() {
            if parent.is_dir() {
                // almost
                parent.to_str().unwrap().to_string() + "/." + p.file_name().unwrap().to_str().unwrap()
            }else{
                // has no parent in path but is not root
                String::from(".") + p.to_str().unwrap()
            }
        }else{
            String::from(".") + p.to_str().unwrap()
        }
    };

    let content: String = fs::read_to_string(filename.clone()).unwrap();
    // println!("In file: {}", filename);
    // println!("contents: {}", contents);

    // for (i, cnt) in content.split('\n').enumerate() {
    //     println!("{}: {}", i, cnt);
    // }

    let mut content_linees = content.split('\n');

    let mut pages = Vec::<(usize, String)>::new();
    for page_num in 1.. {
        let pageend: String = format!("[Page {}]", page_num);
        let mut one_page = String::new();
        while let Some(line) = content_linees.next() {
            one_page += &(String::from("\n") + &line.to_string());
            if line.contains(&pageend) {
                break;
            }
        }

        if one_page.contains(&pageend) {
            pages.push((page_num, one_page));
        }else{
            break;
        }
    }

    let mut current_page_number = 
        if Path::new(&history_filename).exists() {
            let number = fs::read_to_string(history_filename.clone()).unwrap();
            if let Ok(number) = number.trim().parse::<usize>() {
                if 1 <= number && number <= pages.len() {
                    number
                }else{
                    1
                }
            }else{
                1
            }
        }else{
            1
        };
    let mut has_error = false;
    loop {
        if !has_error {
            if let Some((_, one_page)) = pages.iter().nth(current_page_number-1) {
                println!("{}", one_page);
                println!("{:3}/{:3}", current_page_number, pages.len());
            }else{
                let output_errors = format!("invalid page number {}", current_page_number);
                eprintln!("{}", output_errors.red());
            }
        }
        has_error = true;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut input = input.chars();
        let first_character = input.next();
        if Some(':') == first_character {
            let input = input.collect::<String>();
            current_page_number = if let Ok(number) = input.trim().parse::<usize>() {
                if 1 <= number && number <= pages.len() {
                    number
                }else{
                    let output_errors = format!("invalid page number :{}", number);
                    eprintln!("{}", output_errors.red());
                    continue;
                }
            }else{
                let output_errors = format!("invalid input :{}", input);
                eprintln!("{}", output_errors.red());
                continue;
            };

        }else if Some('<') == first_character{
            let num = input.filter(|&c| c == '<').count();
            if current_page_number == 1 {
                let output_errors = format!("this is the first page");
                eprintln!("{}", output_errors.red());
                continue;
            }else{
                current_page_number = if current_page_number > (1+num) {
                    current_page_number - (1+num)
                }else{ 
                    1
                };
            }
        }else if Some('>') == first_character{
            let num = input.filter(|&c| c == '>').count();
            if current_page_number == pages.len() {
                let output_errors = format!("this is the last page");
                eprintln!("{}", output_errors.red());
                continue;
            }else{
                current_page_number = if current_page_number + (1+num) <= pages.len() {
                    current_page_number + (1+num)
                }else{ 
                    pages.len()
                };
            }
        }else if Some('q') == first_character{
            fs::write(history_filename, format!("{}", current_page_number)).unwrap();
            break;
        }else if Some('\n') == first_character {
            if current_page_number == pages.len() {
                let output_errors = format!("this is the last page");
                eprintln!("{}", output_errors.red());
                continue;
            }else{
                current_page_number += 1;
            }
        }else{
            let output_errors = format!("invalid input {}", input.collect::<String>());
            eprintln!("{}", output_errors.red());
            continue;
        }
        has_error = false;
    }
}
