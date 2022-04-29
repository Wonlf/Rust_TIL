fn fi() {
    const N: i32 = 15; //상수 N번째의 피보나치

    let mut t1 = 0;
    let mut t2 = 1;
    let mut temp;

    for _e in 1..=N {
        if _e == N {
            print!("{}번째의 피보나치 수 : {}", N, t1);
        }
        temp = t1 + t2;
        t1 = t2;
        t2 = temp;
    }
}