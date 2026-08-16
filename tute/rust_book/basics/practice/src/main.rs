fn main() {
    let f = celsius_to_fahrenheit(100.0); // 212.0
    let c = fahrenheit_to_celsius(32.0); // 0.0
    println!("f: {}, c: {}", f, c);

    let (min, max, sum) = min_max_sum([1, 2, 3, 4, 5, 6]);
    println!("min: {}, max: {}, sum: {}", min, max, sum);
    let fibb = iter_fibonnacci(10);
    println!("fibb: {}", fibb);
    let result = palindrome_array_seven([1, 2, 3, 4, 3, 2, 1]);
    println!("result: {}", result);
    let (current, step) = collatz(10, 100);
    println!("current: {}, step: {}", current, step);
    twelve_days_of_christmas();
    let rotated = transform_matrix([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    println!("rotated: {:?}", rotated);
    let grade = compute_grade(45);
    println!("grade: {}", grade);
    let arr = int_to_array(123456);
    println!("arr: {:?}", arr);
}

fn celsius_to_fahrenheit(cel: f64) -> f64 {
    return cel * 9.0 / 5.0 + 32.0;
}
fn fahrenheit_to_celsius(faren: f64) -> f64 {
    return (faren - 32.0) * 5.0 / 9.0;
}

fn min_max_sum(arr: [i32; 6]) -> (i32, i32, i32) {
    let mut min = arr[0];
    let mut max = arr[0];
    let mut sum = 0;
    for num in arr {
        if min > num {
            min = num;
        }
        if max < num {
            max = num;
        }
        sum += num;
    }
    return (min, max, sum);
}

fn iter_fibonnacci(n: u32) -> u64 {
    let mut prev: u64 = 0;
    let mut fibb: u64 = 1;
    if n == 0 {
        return 0;
    }
    for _ in 1..n {
        let temp = prev;
        prev = fibb;
        fibb += temp;
    }

    return fibb;
}

// Question 4
// Scope A: x = 32
// Scope B: x = 26, y = 1
// x += 10; this line will throw an error as we are changing the value of an immutable varibale. Add a brief note file for scopes as well

// i used arr.len at first but since we know the array length is 7 we can hard code the values
fn palindrome_array_seven(arr: [i32; 7]) -> bool {
    for i in 0..3 {
        if arr[i] == arr[7 - i - 1] {
            continue;
        } else {
            return false;
        }
    }
    return true;
}

fn collatz(start: u64, limit: u32) -> (u64, u32) {
    println!(
        "Starting Collatz sequence with start = {}, limit = {}",
        start, limit
    );
    let mut step: u32 = 0;
    let mut current = start;
    'collatz: loop {
        step += 1;
        if current == 1 {
            break 'collatz;
        }
        if step == limit {
            break 'collatz;
        } else {
            if current % 2 == 0 {
                current = current / 2;
            } else {
                current = current * 3 + 1;
            }
        }
    }
    return (current, step);
}

// Question 7: Twelve Days of Christmas gifts
fn twelve_days_of_christmas() {
    const GIFTS: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    const DAYS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    for day in 0..12 {
        println!("On the {} day of Christmas", DAYS[day]);
        // rust does not allow any operations inside a string thus we have to use this format
        println!("My true love gave to me");
        for gift in GIFTS[0..=day].iter().rev() {
            println!("{}", gift);
        }
    }
}

fn transform_matrix(arr: [[u32; 3]; 3]) -> [[u32; 3]; 3] {
    let mut rotated = [[0; 3]; 3];
    for row in 0..3 {
        for col in 0..3 {
            rotated[col][2 - row] = arr[row][col];
        }
    }
    rotated
}

fn compute_grade(score: i32) -> i32 {
    let bonus = 5;

    let adjustment = if score > 90 { 10 } else { 5 };

    let status = if score >= 50 { "Pass" } else { "Fail" };
    println!("Student Status: {status}");

    let total = score + adjustment + bonus;

    // Cap the maximum score at 100 using an expression (no `mut` needed)
    if total > 100 { 100 } else { total }
}

fn int_to_array(num: u32) -> [char; 10] {
    let mut arr: [char; 10] = ['0'; 10];
    let mut index = arr.len() - 1;
    let mut number = num;
    while number != 0 {
        arr[index] = ((number % 10 + 48) as u8) as char;
        number = number / 10;
        index -= 1;
    }
    arr
}
