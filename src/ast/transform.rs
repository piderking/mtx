pub trait Transform {
    type Output = Value;

    // How many of the next "expression" it needs
    fn size() -> usize {
        return 1
    }
}

