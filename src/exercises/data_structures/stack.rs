use std::fmt::Debug;

pub struct Stack<T, const N: usize>
where
    T: Copy + Debug,
{
    top: isize,
    size: usize,
    elements: [Option<T>; N],
}

impl<T, const N: usize> Stack<T, N>
where
    T: Copy + Debug,
{
    pub fn new() -> Self {
        Self {
            top: -1,
            size: N,
            elements: [None; N],
        }
    }

    pub fn push(&mut self, val: T) {
        if (self.top as usize) == (self.size - 1) {
            println!("Stack is overflowing. Unable to add new elements to the stack");
        } else {
            self.top += 1;
            self.elements[self.top as usize] = Some(val);
            println!("Value {:#?} pushed to Stack", val);
        }
    }

    pub fn pop(&mut self) {
        if self.top == -1 {
            println!("Stack is empty. Unable to delete");
        } else {
            println!(
                "Value {:#?} popped out of Stack",
                self.elements[self.top as usize].unwrap()
            );
            self.elements[self.top as usize] = None;
            self.top -= 1;
        }
    }

    pub fn display(&self) {
        if self.top == -1 {
            println!("Stack is empty, Nothing to display");
        } else {
            let temp_top = self.top;
            for i in (0..=temp_top).rev() {
                println!("[{}]: {:#?}", i, self.elements[i as usize].unwrap());
            }
        }
    }
}
