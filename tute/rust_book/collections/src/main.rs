// Chapter 8: Common Collections (Vectors, UTF-8 Strings, and Hash Maps)
// Demonstrates all core concepts and end-of-chapter exercises from TRPL.
#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

fn main() {
    println!("==================================================");
    println!("       RUST BOOK CHAPTER 8: COMMON COLLECTIONS    ");
    println!("==================================================");

    vectors_demo();
    strings_demo();
    hashmaps_demo();

    println!("\n==================================================");
    println!("       CHAPTER 8 SUMMARY EXERCISES                ");
    println!("==================================================");

    exercise_1_mean_median_mode();
    exercise_2_pig_latin();
    exercise_3_company_directory();
}

// -----------------------------------------------------------------------------
// 1. VECTORS (Vec<T>)
// -----------------------------------------------------------------------------
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vectors_demo() {
    println!("\n--- 1. VECTORS (Vec<T>) ---");

    // Initialization
    let mut v1: Vec<i32> = Vec::new();
    v1.push(10);
    v1.push(20);
    v1.push(30);

    // Using vec! macro
    let v2 = vec![1, 2, 3, 4, 5];

    // Safe retrieval with .get() vs direct indexing
    let third_index = &v2[2]; // Panics if out of bounds
    println!("v2[2] direct indexing: {third_index}");

    let safe_get: Option<&i32> = v2.get(10); // Returns None safely
    match safe_get {
        Some(val) => println!("Element at index 10: {val}"),
        None => println!("Index 10 is safely out of bounds (returned None)"),
    }

    // Mutable iteration (requires dereferencing `*` to mutate)
    let mut numbers = vec![100, 200, 300];
    for n in &mut numbers {
        *n += 50;
    }
    println!("Mutated vector: {numbers:?}");

    // Heterogeneous collections using Enums
    let row = vec![
        SpreadsheetCell::Int(42),
        SpreadsheetCell::Text(String::from("Revenue")),
        SpreadsheetCell::Float(1234.56),
    ];
    println!("Spreadsheet row: {row:?}");
}

// -----------------------------------------------------------------------------
// 2. UTF-8 STRINGS (String & &str)
// -----------------------------------------------------------------------------
fn strings_demo() {
    println!("\n--- 2. UTF-8 STRINGS ---");

    // Creation
    let mut s1 = String::from("foo");
    s1.push_str("bar"); // Appends string slice without taking ownership
    s1.push('!');       // Appends single char
    println!("s1: {s1}");

    // Concatenation with `+` and Deref Coercion
    // fn add(self, s: &str) -> String
    let hello = String::from("Hello, ");
    let world = String::from("world!");
    let combined = hello + &world; // `hello` is moved, `&world` is coerced to `&str`
    println!("Combined with +: {combined}");

    // Non-consuming formatting
    let formatted = format!("{combined} - {world}");
    println!("Formatted: {formatted}");

    // Multi-byte Unicode Inspection (Hindi word "नमस्ते")
    let hindi = "नमस्ते";
    println!("String: {hindi}");
    println!("Byte length (len): {} bytes", hindi.len()); // 18 bytes
    println!("Unicode scalar values (chars count): {}", hindi.chars().count()); // 6 chars

    print!("Bytes: ");
    for b in hindi.bytes().take(6) {
        print!("{b} ");
    }
    println!("... (truncated)");

    print!("Chars: ");
    for c in hindi.chars() {
        print!("'{c}' ");
    }
    println!();
}

// -----------------------------------------------------------------------------
// 3. HASH MAPS (HashMap<K, V>)
// -----------------------------------------------------------------------------
fn hashmaps_demo() {
    println!("\n--- 3. HASH MAPS ---");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Reading values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{team_name} score: {score}");

    // Iterating over key-value pairs
    for (team, points) in &scores {
        println!("Team {team}: {points}");
    }

    // Updating a value based on the old value (The Entry API)
    let text = "hello world wonderful world hello rust";
    let mut word_counts = HashMap::new();

    for word in text.split_whitespace() {
        // .entry() checks if key exists; .or_insert(0) returns &mut u32
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word counts: {word_counts:?}");
}

// -----------------------------------------------------------------------------
// EXERCISE 1: Mean, Median, and Mode of an Integer List
// -----------------------------------------------------------------------------
fn calculate_stats(numbers: &[i32]) -> (f64, i32, i32) {
    assert!(!numbers.is_empty(), "List must not be empty");

    // 1. Mean
    let sum: i32 = numbers.iter().sum();
    let mean = sum as f64 / numbers.len() as f64;

    // 2. Median (requires sorted vector)
    let mut sorted = numbers.to_vec();
    sorted.sort();
    let median = sorted[sorted.len() / 2];

    // 3. Mode (most frequent value using HashMap Entry API)
    let mut occurrences = HashMap::new();
    for &num in numbers {
        *occurrences.entry(num).or_insert(0) += 1;
    }

    let mode = occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .unwrap();

    (mean, median, mode)
}

fn exercise_1_mean_median_mode() {
    println!("\n[Exercise 1] Mean, Median, Mode Calculator:");
    let dataset = vec![42, 12, 88, 12, 55, 33, 12, 90, 88];
    let (mean, median, mode) = calculate_stats(&dataset);
    println!("Dataset: {dataset:?}");
    println!("Mean:   {mean:.2}");
    println!("Median: {median}");
    println!("Mode:   {mode}");
}

// -----------------------------------------------------------------------------
// EXERCISE 2: Convert Strings to Pig Latin
// -----------------------------------------------------------------------------
// Rules:
// - If word starts with consonant: move first char to end + "ay" ("first" -> "irst-fay")
// - If word starts with vowel (a, e, i, o, u): add "-hay" to the end ("apple" -> "apple-hay")
fn to_pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut result_words = Vec::new();

    for word in text.split_whitespace() {
        let mut chars = word.chars();
        if let Some(first_char) = chars.next() {
            if vowels.contains(&first_char) {
                result_words.push(format!("{word}-hay"));
            } else {
                let rest: String = chars.collect();
                result_words.push(format!("{rest}-{first_char}ay"));
            }
        }
    }

    result_words.join(" ")
}

fn exercise_2_pig_latin() {
    println!("\n[Exercise 2] Pig Latin Translator:");
    let sample = "first apple banana orange umbrella";
    let translated = to_pig_latin(sample);
    println!("Original:   \"{sample}\"");
    println!("Pig Latin:  \"{translated}\"");
}

// -----------------------------------------------------------------------------
// EXERCISE 3: Company Department Directory
// -----------------------------------------------------------------------------
#[derive(Default)]
struct CompanyDirectory {
    // Map of Department Name -> List of Employee Names
    departments: HashMap<String, Vec<String>>,
}

impl CompanyDirectory {
    fn new() -> Self {
        CompanyDirectory {
            departments: HashMap::new(),
        }
    }

    fn add_employee(&mut self, employee: &str, department: &str) {
        let employee_list = self
            .departments
            .entry(department.to_string())
            .or_insert_with(Vec::new);
        employee_list.push(employee.to_string());
    }

    fn get_department_sorted(&self, department: &str) -> Vec<String> {
        match self.departments.get(department) {
            Some(employees) => {
                let mut sorted = employees.clone();
                sorted.sort();
                sorted
            }
            None => Vec::new(),
        }
    }

    fn get_all_sorted(&self) -> HashMap<String, Vec<String>> {
        let mut sorted_company = HashMap::new();
        for (dept, employees) in &self.departments {
            let mut sorted_employees = employees.clone();
            sorted_employees.sort();
            sorted_company.insert(dept.clone(), sorted_employees);
        }
        sorted_company
    }
}

fn exercise_3_company_directory() {
    println!("\n[Exercise 3] Company Department Directory:");
    let mut directory = CompanyDirectory::new();

    directory.add_employee("Sally", "Engineering");
    directory.add_employee("Amir", "Engineering");
    directory.add_employee("Bob", "Sales");
    directory.add_employee("Alice", "Engineering");
    directory.add_employee("Charlie", "Sales");

    println!("Engineering Team (Sorted): {:?}", directory.get_department_sorted("Engineering"));
    println!("Sales Team (Sorted):       {:?}", directory.get_department_sorted("Sales"));
    println!("All Departments (Sorted):  {:?}", directory.get_all_sorted());
}
