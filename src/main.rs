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

// fn main() {
//     let x: i32 = 50;
//     let y: i32 = 10;
//
//     let add_num: i32 = add(x,  y);
//     let sub_num: i32 = sub(x,  y);
//     let imul_num: i32 = imul(x,  y);
//     let div_num: i32 = div(x,  y);
//
//     println!("x + y = {}", add_num);
//     println!("x - y = {}", sub_num);
//     println!("x * y = {}", imul_num);
//     println!("x / y = {}", div_num);
//
// }
//
// fn add(i: i32, j: i32) -> i32 {
//     i + j
// }
//
// fn sub(i: i32, j: i32) -> i32 {
//     i - j
// }
//
// fn imul(i: i32, j: i32) -> i32 {
//     i * j
// }
//
// fn div(i: i32, j: i32) -> i32 {
//     i / j
// }

//In a calm and orderly way 차근차근

//2022-04-18
fn main(){
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6; //기본적으로 변수의 불변성이 적용 컴파일 에러
    // println!("The value of x is: {}", x);


    // let mut x = 5; //mut 키워드를 사용하면 변수를 변경할 수 있게 사용
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    //Rust에서 변수와 상수의 차이는 상수는 mut 키워드를 사용할 수 없음. 불변의 불변인거임


    // let x = 5; //Rust에서 변수를 여러개 선언하면 그 전에 있던 변수를 shadow(덮어쓰기)함
    // let x = x + 1;
    // let x = x * 2;
    // println!("The value of x is: {}", x); //12
    //mut와 재선언의 차이
}