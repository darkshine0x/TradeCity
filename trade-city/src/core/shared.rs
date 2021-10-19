pub mod messaging {
    pub enum Response {
        Placement(i32),
        Execution,
        Cancellation,
        Rejection(String),
        NoOrder
    }
}