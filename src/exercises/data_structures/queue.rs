use std::fmt::Debug;

pub struct Queue<T, const N: usize>
where
    T: Debug + Clone,
{
    front: isize,
    back: isize,
    elements: Vec<T>,
}

impl<T, const N: usize> Queue<T, N>
where
    T: Debug + Clone,
{
    pub fn new() -> Self {
        Self {
            front: -1,
            back: -1,
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, val: T) {
        if self.front == -1 {
            self.front = 0;
            self.back = 0;
        } else {
            self.back += 1;
        }

        self.elements.push(val.to_owned());
        println!("Value {:#?} enqueued", val);
    }

    pub fn dequeue(&mut self) {
        if self.front == -1 {
            println!("Queue is empty. Nothing to delete");
        } else {
            println!("Value {:#?} dequeued", self.elements[self.back as usize]);
            self.back -= 1;

            if self.back == -1 {
                self.front = -1;
            }

            self.elements.pop();
        }
    }

    pub fn display(&self) {
        let temp_back = self.back;

        if self.front == -1 {
            println!("Queue is empty. Nothing to display");
        } else {
            for i in 0..=temp_back {
                println!("[{}]: {:#?}", i, self.elements[i as usize]);
            }
        }
    }
}
