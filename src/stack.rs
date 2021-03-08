#[derive(Debug)]
pub struct Stack {
    key: i32,
    next: Option<Box<Stack>>,
}

pub fn init(key: i32) -> Stack {
    return Stack { key, next: None };
}

// TODO: 今はpushとpopが毎回Stackを返しているのでメソッド化したい
pub fn push(s: Stack, key: i32) -> Stack {
    return Stack {
        key,
        next: Some(Box::new(s)),
    };
}

pub fn pop(stack: Stack) -> (i32, Option<Stack>) {
    let key = stack.key;
    match stack.next {
        Some(s) => return (key, Some(*s)),
        None => return (key, None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let s: Stack = init(0);
        assert_eq!(0, s.key);
        let s = push(s, 10);
        assert_eq!(10, s.key);
        let (k, s) = pop(s);
        assert_eq!(10, k);
        let (k, _) = pop(s.unwrap());
        assert_eq!(0, k);
    }
}
