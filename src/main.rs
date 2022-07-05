fn main() {
    println!("Hello, world!");

    another_function();

    yet_another_function(7);

    print_labeled_measurement(5, 'h');

    let y = 6; // statement

    println!("The value of y is: {y}");

    let z = {
        let x = 3;
        x + 1
    }; // expression

    println!("The value of z is: {z}");
}

fn another_function() {
    println!("Another function.");
}

fn yet_another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
