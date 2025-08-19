pub trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)]
pub struct Grapes {
    pub left: i32,
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.left -= 1;
    }
}

pub fn bunny_nibbles<T: Bite>(food: &mut T) {
    for _i in 0..6 {
        food.bite();
    }
}