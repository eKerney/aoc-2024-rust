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

    fn find_mul_prefix(instuct_str: &str) -> i32 {
        let (mul, mut mul_result) = ("mul(", 0);
        match &instuct_str[..4] == mul {
            true => {
                println!("match mul( - {}", instuct_str);
                mul_result = parse_mul_func(instuct_str);
            }
            false => println!("no match - {}", instuct_str),
        };
        mul_result
    }

    fn parse_mul_func(mul_str: &str) -> i32 {
        let mut acc: i32 = 0;
        let (mut num1, mut num2, mut comma) = (String::from(""), String::from(""), false);
        for (i, c) in mul_str[4..].chars().into_iter().enumerate() {
            match c.is_numeric() {
                true => {
                    match comma {
                        true => num2 = num2 + &c.to_string(),
                        false => num1 = num1 + &c.to_string(),
                    };
                }
                false => {
                    comma = match c == ',' {
                        true => true,
                        false => {
                            if c == ')' {
                                if num1 == "" {
                                    break;
                                }
                                let n1: i32 = match num1.parse() {
                                    Ok(num) => num,
                                    Err(_) => break,
                                };
                                let n2: i32 = match num2.parse() {
                                    Ok(num) => num,
                                    Err(_) => break,
                                };
                                acc = if n1 > 0 && n2 > 0 { acc + n1 * n2 } else { acc };
                                break;
                            } else {
                                break;
                            }
                        }
                    };
                }
            }
        }
        println!("acc _ {}", acc);
        acc
    }

    fn iterate_lines(lines_vec: Vec<&str>) -> i32 {
        let mut sum: i32 = 0;
        for line in lines_vec.iter() {
            println!("{:?}", line);
            for (i, c) in line.chars().into_iter().enumerate() {
                let str_len = match line.len() - i < 20 {
                    true => line.len() - i,
                    false => 20,
                };
                sum = match c == 'm' {
                    true => sum + find_mul_prefix(&line[i..i + str_len]),
                    false => sum,
                }
            }
        }
        sum
    }

    let contents = read_file("./day3/input");
    let lines_vec = lines_to_vec_str(&contents);
    let mul_sum = iterate_lines(lines_vec);
    println!("The result for Day3Part1 = {}", mul_sum);
}
