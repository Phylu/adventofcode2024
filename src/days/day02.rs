pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {
    let reports = prepare_input(content);

    let mut result = 0;
 
    for report in reports {
        let report_correct = check_report(&report);

        if report_correct {
            result += 1;
        }
    }

    return result.to_string();
}


fn task2(content: &String) -> String {
    let reports = prepare_input(content);

    let mut result = 0;

    for report in reports {
        let report_correct = check_report(&report);

        if report_correct {
            result += 1;
        } else {

            for i in 0..report.len() {
                let mut stripped_report = report.clone();
                stripped_report.remove(i);

                let stripped_result = check_report(&stripped_report);
                if stripped_result {
                    result += 1;
                    break;
                }
            }

        }
    }

    return result.to_string();
}


fn check_report(report: &Vec<i32>) -> bool {
    let mut report_correct = true;
        
    let mut signum = 0;
    let mut prev = report[0];

    for num in report[1..].iter() {
        let distance = *num - prev;

        // If this is the first step of the iteration set the order ascending/descending
        if signum == 0 {
            signum = (*num - prev).signum();
        }

        // If the numbers are equal the report is faulty
        if distance == 0 {
            // println!("Report Faulty:{} / {}", prev, *num);
            report_correct = false;
            break;
        }

        // If the numbers are more than 3 apart the report is faulty
        if distance.abs() > 3 {
            // println!("Report Faulty:{} / {}", prev, *num);
            report_correct = false;
            break;
        }

        // If the order is wrong the report is faulty
        if distance.signum() != signum {
            // println!("Report Faulty:{} / {}", prev, *num);
            report_correct = false;
            break;
        }

        prev = *num;
    }

    report_correct
}

fn prepare_input(content: &String) -> Vec<Vec<i32>> {

    let mut reports = Vec::new();

    for line in content.lines() {
        if line != "" {
            let level: Vec<&str> = line.split_whitespace().collect();
            reports.push(level.iter().map(|x| x.parse().unwrap()).collect());
        }
    }
    
    return reports;
}


#[test]
fn test_task1() {
    let content = std::fs::read_to_string("input_test/2.txt").unwrap(); 
    assert_eq!(task1(&content), "2");
}


#[test]
fn test_task2() {
    let content = std::fs::read_to_string("input_test/2.txt").unwrap(); 
    assert_eq!(task2(&content), "4");
}
