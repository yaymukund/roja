use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sender<T> {
    data: Rc<RefCell<VecDeque<T>>>,
}

#[derive(Clone)]
pub struct Receiver<T> {
    data: Rc<RefCell<VecDeque<T>>>,
}

impl<T> Sender<T> {
    pub fn send(&self, msg: T) {
        self.data.borrow_mut().push_back(msg);
    }
}

impl<T> Receiver<T> {
    pub fn recv(&self) -> Option<T> {
        self.data.borrow_mut().pop_front()
    }
}

pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let data = VecDeque::new();
    let data = Rc::new(RefCell::new(data));
    (Sender { data: data.clone() }, Receiver { data: data })
}
