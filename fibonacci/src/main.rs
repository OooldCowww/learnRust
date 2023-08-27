fn main() {
    let mut n_1 = 0;
    let mut n_2 = 1;

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .unwrap();

    let nth = input.trim().parse::<i32>().unwrap();

    let mut count = 0;
    let mut nth_fibo = n_1;
    while count < nth {
        if count == 0 {
            println!("the first Fibonacci number is {nth_fibo}");
            count += 1;
        } else if count == 1 {
            nth_fibo = n_2;
            println!("the second Fibonacci number is {nth_fibo}");
            count += 1;
        } else {
            nth_fibo = n_1 + n_2;
            n_1 = n_2;
            n_2 = nth_fibo;
            println!("the {}th Fibonacci number is {nth_fibo}", count + 1);
            count += 1;
        }
    }
}
