use std::io::stdin;

fn main() {
    let mut nums_str: String = String::new();

    stdin()
        .read_line(&mut nums_str)
        .expect("Failed to read nums");

    let nums_str: Vec<&str> = nums_str.split_ascii_whitespace().collect();

    let mut nums: Vec<i16> = Vec::new();
    let mut current_index: usize = 0;

    for num_str in nums_str {
        let num: i16 = num_str.parse().expect("Failed to get a number");
        nums.push(num);

        if current_index > 0 {
            let mut running_index: usize = current_index;
            while running_index > 0 && num < nums[running_index - 1] {
                nums[running_index] = nums[running_index - 1];
                running_index -= 1;
            }

            nums[running_index] = num;
        }

        current_index += 1;
    }

    let result: Vec<String> = nums.into_iter().map(|i| i.to_string()).collect();
    println!("{}", result.join(" "));
}
