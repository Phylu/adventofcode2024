pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {
    let input = prepare_input(content);
    let mut list_1 = input.0;
    let mut list_2 = input.1;

    list_1.sort();
    list_2.sort();

    let mut result = 0;
 
    for i in 0..list_1.len() {
        let distance = list_1[i] - list_2[i];
        result += distance.abs();
    }

    return result.to_string();
}

fn task2(content: &String) -> String {
    let input = prepare_input(content);

    let mut list_1 = input.0;
    let mut list_2 = input.1;

    list_1.sort();
    list_2.sort();

    let mut result = 0;

    for i in 0..list_1.len() {
        let current = list_1[i];
        let mut ocurrance = 0;
        // This is really unperformant. Should be rewritten. :D
        for j in 0..list_2.len() {
            if current == list_2[j] {
                ocurrance += 1;
            }
        }
        result += current * ocurrance;
    }

    return result.to_string();
}

fn prepare_input(content: &String) -> (Vec<i32>, Vec<i32>) {

    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();

    for line in content.lines() {
        if line != "" {
            let current: Vec<&str> = line.split_whitespace().collect();

            // Add the first number to list_1 and the second to list_2

            //println!("{} / {}", current[0], current[1]);
            list_1.push(current[0].parse().unwrap());
            list_2.push(current[1].parse().unwrap());
        }
    }
    
    return (list_1, list_2);
}


#[test]
fn test_task1() {
    let content = std::fs::read_to_string("input_test/1.txt").unwrap(); 
    assert_eq!(task1(&content), "11");
}


#[test]
fn test_task2() {
    let content = std::fs::read_to_string("input_test/1.txt").unwrap(); 
    assert_eq!(task2(&content), "31");
}
