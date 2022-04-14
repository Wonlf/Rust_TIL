// fn main() {
//     println!("Guess the number!");
//
//     println!("Please input your guess.");
//
//     let mut guess = String::new();
//
//     io::stdin().read_line(&mut guess)
//         .expect("Failed to read line");
//
//     println!("You guessed: {}", guess);
// } //input example, how to get input for num?

fn main() {
    let x: i32 = 50;
    let y: i32 = 10;

    let add_num: i32 = add(x,  y);
    let sub_num: i32 = sub(x,  y);
    let imul_num: i32 = imul(x,  y);
    let div_num: i32 = div(x,  y);

    println!("x + y = {}", add_num);
    println!("x - y = {}", sub_num);
    println!("x * y = {}", imul_num);
    println!("x / y = {}", div_num);

}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn sub(i: i32, j: i32) -> i32 {
    i - j
}

fn imul(i: i32, j: i32) -> i32 {
    i * j
}

fn div(i: i32, j: i32) -> i32 {
    i / j
}