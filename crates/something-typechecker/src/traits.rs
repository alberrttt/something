pub trait TypeCheck<For, With> {
    fn type_check(&mut self, _: For, _: With) -> Result<(), Box<dyn std::error::Error>>;
}
