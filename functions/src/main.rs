fn main() {
    println!("Hello, world!");

    print_label_measurement(5, 'h');
}

fn print_label_measurement(x: i32, unit_label: char) {
    println!("The value of x is: {x}, and the unit label is: {unit_label}");
}
