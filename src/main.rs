use core::str;
use std::{fs, i32};

fn main() {
    fn read_file(file_path: &str) -> String {
        println!("In file {}", file_path);
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        contents
    }

    fn lines_to_vec_str(contents: &String) -> Vec<&str> {
        let all_lines: Vec<&str> = contents.lines().collect();
        return all_lines;
    }

    fn vec_str_to_num(vec_str: Vec<&str>) -> Vec<Vec<i32>> {
        let mut all_lines_nums: Vec<Vec<i32>> = Vec::new();
        for line in vec_str.iter() {
            let input = line
                .split_whitespace()
                .map(|x| x.trim().parse::<i32>().unwrap_or(0))
                .collect::<Vec<_>>();
            all_lines_nums.push(input);
        }
        return all_lines_nums;
    }

    fn get_char_matrix(lines_vec: Vec<&str>) -> Vec<Vec<char>> {
        let mut y: Vec<Vec<char>> = Vec::new();
        for line in lines_vec.iter() {
            println!("{}", line);
            let x: Vec<_> = line.chars().collect();
            y.push(x);
        }
        y
    }

    fn find_mul_prefix(instuct_str: &str) -> () {
        let mul = "mul(";
        match &instuct_str[..4] == mul {
            true => {
                println!("match mul( - {}", instuct_str);
                parse_mul_func(instuct_str);
            }
            false => println!("no match - {}", instuct_str),
        };
    }

    fn parse_mul_func(mul_str: &str) -> () {
        let mut num1 = String::from("");
        for (i, c) in mul_str[4..].chars().into_iter().enumerate() {
            match c.is_numeric() {
                true => {
                    num1 = num1 + &c.to_string();
                    println!("{}", num1);
                }
                false => {
                    // check for comma
                }
            }
            println!("{} _ {}", c, c.is_numeric());
        }
    }

    fn iterate_lines(lines_vec: Vec<&str>) -> () {
        for line in lines_vec.iter() {
            println!("{:?}", line);
            for (i, c) in line.chars().into_iter().enumerate() {
                // println!("{} - {} - {} _ {} ", i, c, line.len(), line.len() - i);
                let str_len = match line.len() - i < 20 {
                    true => line.len() - i,
                    false => 20,
                };
                // println!("str_len _ {}", str_len);
                match c == 'm' {
                    // true => println!("mul string - {:?}", &line[i..i + 4]),
                    true => find_mul_prefix(&line[i..i + str_len]),
                    false => (),
                }
            }
        }
    }

    let contents = read_file("./day3/testInput");
    let lines_vec = lines_to_vec_str(&contents);
    iterate_lines(lines_vec);
    // let chars = get_char_matrix(lines_vec);

    // println!("The result for Day1Part2 = {}", sim);
}
