pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn times(&self, times: u32) -> Self {
        Rectangle {
            width: self.width * times,
            height: self.height * times,
        }
    }

    pub fn can_hold(&self, other: &Self) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    pub fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
