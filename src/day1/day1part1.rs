use std::{char, env, fs, i16, i32};

fn main() {
    println!("Hello, world!");

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

    fn split_vec_sort(vecs: Vec<Vec<i32>>, index: usize) -> Vec<i32> {
        let mut flat_vec: Vec<i32> = vecs.iter().map(|n| n[index]).collect();
        flat_vec.sort();
        flat_vec
    }

    fn get_vec_diff(v1: Vec<i32>, v2: Vec<i32>) -> i32 {
        let mut diff: i32 = 0;
        for (i, d0) in v1.iter().enumerate() {
            let d1 = v2[i];
            diff = diff + (d0 - d1).abs();
            // println!("{} - {} - {} - {}", i, d0, d1, diff);
        }
        diff
    }

    let contents = read_file("./day1/input");
    let lines_vec = lines_to_vec_str(&contents);
    let vec_num = vec_str_to_num(lines_vec);

    let v1 = split_vec_sort(vec_num.clone(), 0);
    let v2 = split_vec_sort(vec_num.clone(), 1);

    let diff = get_vec_diff(v1, v2);
    println!("The result for Day1Part1 = {}", diff);
}
