#[derive(Debug)]
pub struct Queue {
    data: [i32; 256],
    top: Option<usize>,
    rear: Option<usize>,
}

impl Queue {
    pub fn init() -> Queue {
        Queue {
            data: [0; 256],
            top: None,
            rear: None,
        }
    }

    pub fn enqueue(&mut self, i: i32) {
        if self.top.is_none() & self.rear.is_none() {
            self.top = Some(0);
            self.rear = Some(0);
        } else if self.data.len() < self.rear.unwrap() {
            panic!("QUEUE IS FULL");
        }
        self.data[self.rear.unwrap()] = i;
        self.rear = Some(self.rear.unwrap() + 1);
    }

    pub fn dequeue(&mut self) -> Result<i32, &'static str> {
        if self.top.is_none() {
            return Err("NO DATA");
        }
        let i = self.data[self.top.unwrap()];
        self.top = Some(self.top.unwrap() + 1);
        return Ok(i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut queue = Queue::init();
        queue.enqueue(0);
        queue.enqueue(1);
        let i = queue.dequeue().unwrap();
        assert_eq!(0, i);
        queue.enqueue(2);
        let i = queue.dequeue().unwrap();
        assert_eq!(1, i);
    }
}
