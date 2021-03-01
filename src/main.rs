use std::io;
use std::io::Write;

fn main() {
    let mut a: [i32; 100] = [0; 100];
    print!("最大次数を入力して下さい: ");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n: i32 = s.trim().parse().unwrap();
    println!("{}", n);

    println!("各項の係数を入力してください");
    for i in 0..=n {
        print!("次数が{}の項の係数: ", i);
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        a[i as usize] = s.trim().parse().unwrap();
    }
    print!("x の値を入力してください: ");
    io::stdout().flush().unwrap();
    let x: i32 = {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().unwrap()
    };
    println!("{}", (0..33).map(|_| "-").collect::<String>());
    println!("入力した多項式（但し, a^b はaのb乗を表す）");
    for i in (1..=n).rev() {
        print!("{}*{}^{} + ", a[i as usize], x, i);
    }
    println!("{}*{}^{}", a[0], x, 0);

    let mut k = a[n as usize];
    for i in (0..=n - 1).rev() {
        k = k * x + a[i as usize];
    }

    println!("\n答え: {}", k);
}
