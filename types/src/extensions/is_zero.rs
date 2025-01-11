pub trait IsZero {
    fn is_zero(&self) -> bool;
}

impl IsZero for u8 {
    fn is_zero(&self) -> bool {
        *self == 0
    }
}
