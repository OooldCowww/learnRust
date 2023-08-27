fn main() {
    let mut input = String::new();

    println!("Enter a Fahrenheit temperature");

    std::io::stdin()
        .read_line(&mut input)
        .unwrap();

    let number: f32 = input.trim().parse::<f32>().unwrap();

    let c_temp: f32 = (number - 32.0) * 5.0 / 9.0;

    println!("the Celsius temperature is {}", c_temp);
}
