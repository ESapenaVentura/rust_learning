fn main() {
    print_labeled_measurement(5, 'h');
    let mut x: i32 = 5;
    let y: i32 = {
        x = x + 2;
        sum_2(x)
    };
    println!("x is {x}; y is {y}");
}

fn sum_2(value: i32) -> i32 {
    value + 2
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
