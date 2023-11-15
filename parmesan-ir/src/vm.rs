pub struct Flags {
    pub debug: bool,
    pub each_ir: bool,
}
pub trait Vm {
    fn execute(&self) {}
    fn get_flags(&self) -> Flags {
        Flags {
            debug: false,
            each_ir: false,
        }
    }
}
