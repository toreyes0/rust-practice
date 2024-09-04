use std::collections::HashMap;
use std::io::{self, Write};

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // vectors
    // stores values of the same type
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    //let does_not_exist = &v[100]; // panics
    let does_not_exist = v.get(100); // doesn't panic, returns None

    // iterating
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // dereference
        println!("{i}");
    }

    // storing multiple types using enum
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // dropping a vector drops its elements
    {
        let v = vec![1, 2, 3, 4];
    } // <- v goes out of scope and is freed

    // strings
    /* 
    2 most common strings
    `str` - string slice, implemented in Rust's core language
    `String` - provided by the standard library
    */

    let mut s = String::new();
    let data = "initial contents"; // string literal
    // both do the same, just a matter of preference
    let s = data.to_string();
    let s = String::from(data);
    
    // appending
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    // appending only one character
    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    // concatenating
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // compiler coerces 2nd and succeeding elements' type
    // from &String to &str (deref coercion)
    let s3 = s1 + &s2; // s3 takes ownership of s1
    // <- s1 is no longer valid 
    println!("{s3}");
    // using format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    // indexing
    /* 
    Rust strings don't support indexing (using standard library) because:
    - each character is encoded in UTF-8 and takes 2 bytes of storage
    - there are 3 ways to look at strings - bytes, scalar values, and grapheme clusters
    - indexing operations are always expected to take O(1) time, which is not guaranteed on a String
    */

    // hash maps
    // stores keys and values (i.e. dictionary in Python, object in JavaScript)
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // overwriting
    scores.entry(String::from("Yellow")).or_insert(50); // adds if key doesn't exist
    scores.entry(String::from("Blue")).or_insert(50); // doesn't update since key already exists

    // accessing value
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    // iterating
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // updating a value based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() { // returns an iterator over subslices
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    // excercise
    // median and mode
    let mut numbers = vec![9, 3, 6, 3, 7, 8, 1];
    let (median, mode) = find_median_and_mode(&mut numbers);

    match median {
        Some(m) => println!("Median: {}", m),
        None => println!("No median found"),
    }

    match mode {
        Some(m) => println!("Mode: {}", m),
        None => println!("No mode found"),
    }

    // pig latin
    let sentence = "hello apple world";
    let pig_latin_sentence = pig_latin(sentence);
    println!("{}", pig_latin_sentence);

    // employee assignments
    //employee_assignments();
}

fn find_median_and_mode(numbers: &mut Vec<i32>) -> (Option<i32>, Option<i32>) {
    numbers.sort();

    // find the median
    let median = if numbers.len() % 2 == 0 {
        // even elements = median is the average of the two middle elements
        let mid = numbers.len() / 2;
        let left = numbers[mid - 1];
        let right = numbers[mid];
        Some((left + right) / 2)
    } else {
        // odd elements = median is the middle element
        let mid = numbers.len() / 2;
        Some(numbers[mid])
    };

    // count occurrences for the mode
    let mut counts = HashMap::new();
    for &number in numbers.iter() {
        *counts.entry(number).or_insert(0) += 1;
    }

    // find the mode
    let mode = counts
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(&number, _)| number);

    (median, mode)
}

fn pig_latin_word(word: &str) -> String {
    let vowels = "aeiouAEIOU";
    
    if word.is_empty() {
        return word.to_string();
    }
    
    // check if first character is a vowel
    if vowels.contains(word.chars().next().unwrap()) {
        return format!("{}-hay", word);
    } else {
        // move first consonant to the end and add "ay"
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();
        let rest_of_word: String = chars.collect();
        format!("{}-{}ay", rest_of_word, first_char)
    }
}

fn pig_latin(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .map(|word| pig_latin_word(word))
        .collect::<Vec<String>>()
        .join(" ")
}

fn employee_assignments() {
    // HashMap to store departments and their employee names
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Options:");
        println!("1. Add employee");
        println!("2. List employees by department");
        println!("3. List all employees");
        println!("4. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap(); // ensures prompt is printed

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();
        let option = option.trim();

        match option {
            "1" => {
                // add employee to a department
                print!("Enter employee name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();

                print!("Enter department: ");
                io::stdout().flush().unwrap();
                let mut department = String::new();
                io::stdin().read_line(&mut department).unwrap();
                let department = department.trim().to_string();

                let entry = company.entry(department).or_insert_with(Vec::new);
                entry.push(name);
                println!("Employee added.");
            }
            "2" => {
                // list employees by department
                print!("Enter department: ");
                io::stdout().flush().unwrap();
                let mut department = String::new();
                io::stdin().read_line(&mut department).unwrap();
                let department = department.trim();

                if let Some(employees) = company.get(department) {
                    let mut sorted_employees = employees.clone();
                    sorted_employees.sort();
                    println!("Employees in {}: {:?}", department, sorted_employees);
                } else {
                    println!("Department not found.");
                }
            }
            "3" => {
                // list all employees
                let mut departments: Vec<_> = company.keys().collect();
                departments.sort();
                
                for dept in departments {
                    if let Some(employees) = company.get(dept) {
                        let mut sorted_employees = employees.clone();
                        sorted_employees.sort();
                        println!("Employees in {}: {:?}", dept, sorted_employees);
                    }
                }
            }
            "4" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}
