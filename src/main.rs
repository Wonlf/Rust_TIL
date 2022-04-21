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
//fn main(){
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
//}

//2022-04-19
//fn main(){
    //rust에도 데이터 타입에 따라 최소 크기와 최대 크기가 정해짐
    // let x: f64 = 2.0; // f64 1배수의 정밀도인 부동소수점
    // let y: f32 = 3.0; // f32 2배수의 정밀도인 부동소수점
    //
    // // addition
    // let sum = 5 + 10;
    //
    // // subtraction
    // let difference = 95.5 - 4.3;
    //
    // // multiplication
    // let product = 4 * 30;
    //
    // // division
    // let quotient = 56.7 / 32.2;
    //
    // // remainder
    // let remainder = 43 % 5;
    //
    // let t = true;
    //
    // let f: bool = false; // with explicit type annotation
    //
    // let c = 'z';
    // let z = 'ℤ'; //특수문자와
    // let heart_eyed_cat = '😻'; //이모지 사용가능
    //
    //
    // let tup = (500, 6.4, 1); //튜플형
    //
    // let (x, y, z) = tup; //튜플 추출
    //
    //
    // let x: (i32, f64, u8) = (500, 6.4, 1); //튜플의 별명 같은걸 추가
    //
    // let five_hundred = x.0; // 배열 처럼 사용 가능
    //
    // let six_point_four = x.1;
    //
    // let one = x.2;
    //
    //
    // let a = [1, 2, 3, 4, 5];// 배열
    // let first = a[0]; // 접근

//}

//2022-04-21
fn main() {
    fn plus_one(x: i32) -> i32 {
        x + 1 //반환값은 무조건 세미콜론 없음
    }

    let y = {
        let x = 3;
        x + 1 //표현식의 반환값도 세미콜론이 없지만
    }; //표현식의 마무리는 무조건 세미콜론

    println!("Hello, world!");

    another_function(1, 2);
}


fn another_function(x: i32, y: i32) { //함수 인자값
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}