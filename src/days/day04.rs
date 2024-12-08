use std::char;

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {
    let input = prepare_input(content);
    let mut result = 0;

    for x in 0..input.len() {
        for y in 0..input[0].len() {
            result += check_xmas(&input, x as i32, y as i32);
        }
    }

    return result.to_string();
}

fn task2(content: &String) -> String {
    let input = prepare_input(content);
    let mut result = 0;

    for x in 0..input.len() {
        for y in 0..input[0].len() {
            result += check_x_mas(&input, x as i32, y as i32);
        }
    }

    return result.to_string();
}

fn prepare_input(content: &String) -> Vec<Vec<char>>{

    let mut characters: Vec<Vec<char>> = Vec::new();

    for line in content.lines() {
        let mut current_line: Vec<char> = Vec::new();
        for char in line.chars() {
            current_line.push(char);
        }
        characters.push(current_line);
    }
    
    return characters;
}

fn check_xmas(characters: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {    
    let mut result = 0;

    if check_xmas_direction(characters, x, y, 1, 0) {
        result += 1;
    }

    if check_xmas_direction(characters, x, y, 0, 1) {
        result += 1;
    }

    if check_xmas_direction(characters, x, y, 1, 1) {
        result += 1;
    }

    if check_xmas_direction(characters, x, y, -1, 0) {
        result += 1;
    }

    if check_xmas_direction(characters, x, y, 0, -1) {
        result += 1;
    }

    if check_xmas_direction(characters, x, y, -1, -1) {
        result += 1;
    }

    if check_xmas_direction(characters, x, y, -1, 1) {
        result += 1;
    }

    if check_xmas_direction(characters, x, y, 1, -1) {
        result += 1;
    }
    
    result
}

fn check_xmas_direction(characters: &Vec<Vec<char>>, x: i32, y: i32, x_direction: i32, y_direction: i32) -> bool {
    let empty: Vec<char> = Vec::new();

    let char_1 = characters.
        get(x as usize).unwrap_or(&empty).
        get(y as usize).unwrap_or(&'Z');
    let char_2 = characters.
        get((x + 1 * x_direction) as usize).unwrap_or(&empty).
        get((y + 1 * y_direction) as usize).unwrap_or(&'Z');
    let char_3 = characters.
        get((x + 2 * x_direction) as usize).unwrap_or(&empty).
        get((y + 2 * y_direction) as usize).unwrap_or(&'Z');
    let char_4 = characters.
        get((x + 3 * x_direction) as usize).unwrap_or(&empty).
        get((y + 3 * y_direction) as usize).unwrap_or(&'Z');

        
        if char_1 != &'X' {
            return false;
        }
        
        if char_2 != &'M' {
            return false;
        }
        
        if char_3 != &'A' {
            return false;
        }
        
        if char_4 != &'S' {
            return false;
        }
        
    //println!("{}-{} : {} / {} / {} / {}", x, y, char_1, char_2, char_3, char_4);
    true
}

fn check_x_mas(characters: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {    
    // 2.3
    // .1.
    // 4.5
    
    let empty: Vec<char> = Vec::new();

    let char_1 = characters.
        get(x as usize).unwrap_or(&empty).
        get(y as usize).unwrap_or(&'Z');

    if char_1 != &'A' {
        return 0
    }

    let char_2 = characters.
        get((x - 1) as usize).unwrap_or(&empty).
        get((y - 1) as usize).unwrap_or(&'Z');
    let char_3 = characters.
        get((x - 1) as usize).unwrap_or(&empty).
        get((y + 1) as usize).unwrap_or(&'Z');
    let char_4 = characters.
        get((x + 1) as usize).unwrap_or(&empty).
        get((y - 1) as usize).unwrap_or(&'Z');
    let char_5 = characters.
        get((x + 1) as usize).unwrap_or(&empty).
        get((y + 1) as usize).unwrap_or(&'Z');

    if ((char_2 == &'M' && char_5 == &'S') || (char_2 == &'S' && char_5 == &'M')) &&
       ((char_3 == &'M' && char_4 == &'S') || (char_3 == &'S' && char_4 == &'M')) {
        //println!("{}-{} : {} / {} / {} / {} / {}", x, y, char_1, char_2, char_3, char_4, char_5);
        return 1
    }

    return 0
}

#[test]
fn test_task1() {
    let content = std::fs::read_to_string("input_test/4.txt").unwrap(); 
    assert_eq!(task1(&content), "18");
}


#[test]
fn test_task2() {
    let content = std::fs::read_to_string("input_test/4.txt").unwrap(); 
    assert_eq!(task2(&content), "9");
}
