#[macro_export]
macro_rules! tkn_recover {
    (eot $expr:expr) => {
        match $expr {
            Ok(x) => Ok(x),
            Err(_) | Recoverable => Recoverable,
        }
    };
}
#[macro_export]
/// imitates the `matches!` macro
macro_rules! peek_matches {
    ($self:ident, $($pat:pat_param)|+) => {
        match $self.peek() {
            Ok(token) => match token {
                $($pat)|+ => true,
                _ => false,
            },
            _ => false,
        }
    };
}
