mod lib;
mod queue;
mod stack;

fn main() {
    lib::make_list();

    // stack
    let s: stack::Stack = stack::init(0);
    let s: stack::Stack = stack::push(s, 1);
    let s: stack::Stack = stack::push(s, 2);
    println!("{:?}", s);

    // queue
    let mut q = queue::Queue::init();
    q.enqueue(0);
    let i = q.dequeue();
    println!("{}", i.unwrap());
}
