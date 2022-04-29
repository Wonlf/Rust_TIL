fn main() {
    //현재 섭씨온도 : 20°C
    //현재 화씨온도 : 68°F
    let now_celsius = 20;
    let now_fahrenheit = 68;

    let fahrenheit = celsius2fahrenheit(now_celsius);
    println!("{:.4}°F", fahrenheit);

    let celsius = fahrenheit2celsius(now_fahrenheit);
    println!("{:.4}°C", celsius);
}

fn celsius2fahrenheit(temperature:i32) -> f64 { //섭씨에서 화씨로
    //섭씨에서 화씨로 바꾸는 공식 = 섭씨 * 1.8 + 32
    let temperature:f64 = temperature as f64; //섭씨를 정수에서 소수점으로
    let value:f64 = 32 as f64; //공식 중, 정수를 소수점으로

    return temperature * 1.8 + value;
}

fn fahrenheit2celsius(temperature:i32) -> f64 { //섭씨에서 화씨로
    //화씨에서 섭씨로 바꾸는 공식 = 화씨 - 32 / 1.8
    let temperature:f64 = temperature as f64; //화씨를 정수에서 소수점으로
    let value:f64 = 32 as f64; //공식 중, 정수를 소수점으로

    return (temperature - value) / 1.8;
}