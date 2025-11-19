pub enum ControlSignal {
    Shutdown,
    Restart,
    Error {
        source: &'static str,
        msg: String,
        severity: ErrorSeverity,
    },
}

pub enum ErrorSeverity {
    Warning,
    Critical,
}
