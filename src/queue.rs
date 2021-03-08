#[derive(Debug)]
pub struct Queue {
    key: i32,
    before: Option<Box<Queue>>,
}

struct QueueHead {
    top: Option<Box<Queue>>,
    rear: Option<Box<Queue>>,
}

pub fn init() -> QueueHead {
    return QueueHead {
        top: None,
        rear: None,
    };
}

pub fn enqueue(mut q: QueueHead, key: i32) -> QueueHead {
    let new = Queue { key, before: None };
    let mut w = &*q.rear.as_ref().unwrap();
    if !q.rear.is_none() {
        q.rear = Some(Box::new(new));
        w.before = Some(Box::new(new));
    } else {
        q.rear = Some(Box::new(new));
        q.top = Some(Box::new(new));
    }
    return q;
}

pub fn dequeue(q: QueueHead) -> i32 {
    if q.top.is_none() {
        println!("キューが空です");
        return 0;
    } else {
        let w = *q.top.unwrap();
        let key = w.key;
        q.top = w.before;
        if q.top.is_none() {
            q.rear = None;
        }
        return key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let qh = init();
        let qh = enqueue(qh, 0);
        assert_eq!(0, *qh.top.unwrap().key);
        let (k, s) = dequeue(qh);
        assert_eq!(0, k);
        let (k, _) = dequeue(qh);
        assert_eq!(1, k);
    }
}
