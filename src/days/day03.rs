use itertools::join;
use regex::Regex;

const MULTIPLY: &str = r"mul\((?P<x>\d+),(?P<y>\d+)\)";

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {
    let input = prepare_input(content);

    let mut result = 0;

    for (x, y) in input {
        result += x * y;
    }

    return result.to_string();
}

fn task2(content: &String) -> String {
    let input = prepare_input_2(content);

    let mut result = 0;

    for (x, y) in input {
        result += x * y;
    }

    return result.to_string();
}

fn prepare_input(content: &String) -> Vec<(i32, i32)> {

    let mut result  = Vec::new();
    let re = Regex::new(MULTIPLY).unwrap();

    for line in content.lines() {
        let captures = re.captures_iter(line);

        for capture in captures {
            let x: i32 = capture.name("x").unwrap().as_str().parse().unwrap();
            let y: i32 = capture.name("y").unwrap().as_str().parse().unwrap();
            result.push((x, y));
        }
    }


    result
}

fn prepare_input_2(content: &String) -> Vec<(i32, i32)> {

    let mut result  = Vec::new();
    let re = Regex::new(MULTIPLY).unwrap();
    let mut input = join(content.lines(), "");

        let mut collection = true;

        while input != "" {
            let delimiter;
            if collection {
                delimiter = "don't()";
            } else {
                delimiter = "do()";
            }
            let (first, second) = input.split_once(delimiter).unwrap_or((&input, ""));

            // println!("{} / {}", first, second);

            if collection {
                let captures = re.captures_iter(first);

                for capture in captures {
                    let x: i32 = capture.name("x").unwrap().as_str().parse().unwrap();
                    let y: i32 = capture.name("y").unwrap().as_str().parse().unwrap();
                    result.push((x, y));
                }
            }

            input = second.to_owned();
            collection = !collection;

            // println!("collection: {}", collection);
        }

    result
}


#[test]
fn test_task1() {
    let content = std::fs::read_to_string("input_test/3.txt").unwrap(); 
    assert_eq!(task1(&content), "161");
}


#[test]
fn test_task2() {
    let content = std::fs::read_to_string("input_test/3_1.txt").unwrap(); 
    assert_eq!(task2(&content), "48");
}
