use std::io;
use std::io::Write;

pub fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        return x;
    }
    return gcd(y, x % y);
}

struct Vertex {
    name: i32,
    key: i32,
    next: Option<Box<Vertex>>,
}

pub fn make_list() {
    println!("各要素を入力してください");
    let mut top: Option<Box<Vertex>> = None;
    loop {
        print!("nameの値を入力してください: ");
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let n: i32 = s.trim().parse().unwrap();
        if n < 0 {
            break;
        }
        print!("keyの値を入力してください: ");
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let k: i32 = s.trim().parse().unwrap();
        let new: Option<Box<Vertex>> = Some(Box::new(Vertex {
            name: n,
            key: k,
            next: top,
        }));
        // MEMO: ライフタイムがこのループ内なので、ループ外に出たとしても存在させる方法を調べる必要がある
        top = new; // TODO: ここで型を合わせないといけない
    }
    println!();
    println!("入力したデータ（[name, key]）の順番でヘッドに近い構造体から出力");
    while top.is_some() {
        let t = top.unwrap();
        print!("[{}, {}] ", t.name, t.key);
        io::stdout().flush().unwrap();
        top = t.next;
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn test_gcd() {
        let x: i32 = 27;
        let y: i32 = 18;
        assert_eq!(9, gcd(x, y));
    }
}
