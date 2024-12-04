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
        // set safe at 0
        let mut safe_report_count: i32 = 0;
        for v in vec_num.iter() {
            println!("{:?}", v);
            let mut level_safe: bool = true;
            let mut error_count: i32 = 0;
            let (mut dir0, mut dir1, mut dir_init) = (0, 0, 0);
            for (i, d) in v.iter().enumerate() {
                // skip last iteration to avoid double counting errors
                println!("dir_init: {}", dir_init);
                dir_init = match i == 0 {
                    true => d - (v[i + 1]),
                    false => dir_init,
                };
                if i != v.len() - 1 {
                    dir0 = match i == (v.len() - 1) {
                        false => d - v[i + 1],
                        true => dir0,
                    };
                    dir1 = match i == (v.len() - 2) || i == (v.len() - 1) {
                        false => v[i + 1] - v[i + 2],
                        true => dir1,
                    };
                    if level_safe == false {
                        if (dir_init > 0) != (dir1 > 0) {
                            println!("2first case - dir0 - {} - dir1 - {} ", dir0, dir1);
                            error_count += 1;
                        } else if dir0.abs() < 1 || dir0.abs() > 3 {
                            println!("2dir0 out range - dir0 - {} ", dir0);
                            error_count += 1;
                        }
                        // } else if dir1.abs() < 1 || dir1.abs() > 3 {
                        //     println!("2dir1 out range - dir1 - {} ", dir1);
                        //     error_count += 1;
                        // }
                    };
                    if level_safe == true {
                        if (dir0 > 0) != (dir1 > 0) {
                            println!("first case - dir0 - {} - dir1 - {} ", dir0, dir1);
                            level_safe = false;
                            error_count = error_count + 1;
                        } else if dir0.abs() < 1 || dir0.abs() > 3 {
                            println!("dir0 out range - dir0 - {} ", dir0);
                            level_safe = false;
                            error_count = error_count + 1;
                        }
                        // else if dir1.abs() < 1 || dir1.abs() > 3 {
                        //     println!("dir1 out range - dir1 - {} ", dir1);
                        //     level_safe = false;
                        //     error_count = error_count + 1;
                        // }
                        else {
                            level_safe = true;
                        };
                    };

                    // println!(
                    //     "{} - {} - {} - {} - {} - {} - {}",
                    //     i,
                    //     d,
                    //     dir0.abs(),
                    //     dir1.abs(),
                    //     level_safe,
                    //     safe_report_count,
                    //     error_count
                    // );
                };
            }

            safe_report_count = match level_safe {
                true => safe_report_count + 1,
                false => safe_report_count,
            };
            safe_report_count = match error_count == 1 {
                true => safe_report_count + 1,
                false => safe_report_count,
            };
            println!("error_count - {}", error_count);
        }
        safe_report_count
    }

    let contents = read_file("./day2/input");
    let lines_vec = lines_to_vec_str(&contents);
    let vec_num = vec_str_to_num(lines_vec);

    // check_all_num_pos(vec_num);
    println!("Safe Reports = {}", check_reports(vec_num));
    // println!("The result for Day1Part2 = {}", sim);
}
