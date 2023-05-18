pub trait TypeCheck<For> {
    fn type_check(&mut self, _: For) -> Result<(), Box<dyn std::error::Error>>;
}

