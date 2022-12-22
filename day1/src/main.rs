fn main() {
    solve_day_1();
}

fn solve_day_1() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    let mut totals: Vec<i32> = Vec::new();

    for line in lines {
        if line.is_empty() {
            totals.push(sum);
            sum = 0;
            continue;
        }
        let number: i32 = line.parse::<i32>().unwrap();
        sum += number;
    }
    totals.push(sum);
    totals.sort_by(|a, b| b.cmp(a));
    println!("largest total: {}", totals[0]);

    // part 2

    // make new array with top 3 largest totals
    let mut top_3: Vec<i32> = Vec::new();

    for i in 0..3 {
        top_3.push(totals[i]);
    }

    // add top 3
    let sum = top_3[0] + top_3[1] + top_3[2];

    // print sum
    println!("sum of top 3: {}", sum);
}
