#[derive(Debug)]
pub struct Rectangle {
    pub width: u16,
    pub height: u16
}

// method syntax
impl Rectangle {
    pub fn area_of_rectangle(&self) -> u16{
        self.height * self.width
    }
}

pub fn area_rectangle(rect: &Rectangle) -> u16 {
    rect.height * rect.width
}