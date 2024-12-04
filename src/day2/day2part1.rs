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

    fn check_reports(vec_num: Vec<Vec<i32>>) -> i32 {
        let mut safe_report_count: i32 = 0;
        // set safe at 0
        for v in vec_num.iter() {
            println!("{:?}", v);
            let mut level_safe: bool = false;
            let (mut dir0, mut dir1) = (0, 0);
            for (i, d) in v.iter().enumerate() {
                // println!("level_safe - {}", level_safe);
                dir0 = match i == (v.len() - 1) {
                    false => d - v[i + 1],
                    true => dir0,
                };
                dir1 = match i == (v.len() - 2) || i == (v.len() - 1) {
                    false => v[i + 1] - v[i + 2],
                    true => dir1,
                };
                if (dir0 > 0) != (dir1 > 0) {
                    println!("first case - dir0 - {} - dir1 - {} ", dir0, dir1);
                    level_safe = false;
                    break;
                } else if dir0.abs() < 1 || dir0.abs() > 3 {
                    println!("dir0 out range - dir0 - {} ", dir0);
                    level_safe = false;
                    break;
                } else if dir1.abs() < 1 || dir1.abs() > 3 {
                    println!("dir1 out range - dir1 - {} ", dir1);
                    level_safe = false;
                    break;
                } else {
                    level_safe = true;
                }
                println!(
                    "{} - {} - {} - {} - {}",
                    d,
                    dir0.abs(),
                    dir1.abs(),
                    level_safe,
                    safe_report_count
                );
            }

            safe_report_count = match level_safe {
                true => safe_report_count + 1,
                false => safe_report_count,
            };
        }
        safe_report_count
    }

    fn check_all_num_pos(vec_num: Vec<Vec<i32>>) -> i32 {
        for v in vec_num.iter() {
            // println!("{:?}", v);
            for (i, &d) in v.iter().enumerate() {
                match d >= 0 {
                    true => continue,
                    false => println!("neg: {}", d),
                }
            }
        }
        1
    }

    let contents = read_file("./day2/input");
    let lines_vec = lines_to_vec_str(&contents);
    let vec_num = vec_str_to_num(lines_vec);

    // check_all_num_pos(vec_num);
    println!("Safe Reports = {}", check_reports(vec_num));
}
