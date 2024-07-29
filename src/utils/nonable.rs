

pub trait Nonable {
    fn none() -> Self;
    fn is_none(&self) -> bool;
}

impl Nonable for () {
    fn none() -> Self { () }
    fn is_none(&self) -> bool { true }
}

impl<A: Nonable, B: Nonable> Nonable for (A, B) {
    fn none() -> Self {
        (A::none(), B::none())
    }
    fn is_none(&self) -> bool {
        self.0.is_none() && self.1.is_none()
    }
}

