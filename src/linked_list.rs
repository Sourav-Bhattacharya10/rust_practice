pub enum LinkedList {
    Element(i32, Box<LinkedList>),
    Nil
}

impl LinkedList {
    pub fn new() -> Self {
        Self::Nil
    }

    pub fn prepend(self, element: i32) -> Self {
        Self::Element(element, Box::new(self))
    }

    pub fn append(self, element: i32) -> Self {
        match self {
            LinkedList::Element(_head, tail) => {
                Self::Element(_head, Box::new(tail.append(element)))
            },
            LinkedList::Nil => {
                Self::Element(element, Box::new(self))
            },
        }
    }

    pub fn display(self) {
        match self {
            LinkedList::Element(head, tail) => {
                print!("{} -> ", head);
                tail.display();
            },
            LinkedList::Nil => print!("Nil"),
        }
    }
}