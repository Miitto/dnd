pub trait IsFalse {
    fn is_false(&self) -> bool;
}

impl IsFalse for bool {
    fn is_false(&self) -> bool {
        !self
    }
}
