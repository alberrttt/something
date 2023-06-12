pub enum Result<Ok, Err> {
    Ok(Ok),
    Recoverable,
    Err(Err),
}
