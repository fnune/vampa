pub struct Diagnostic {
  pub level: DiagnosticLevel,
  pub message: String,
}

pub enum DiagnosticLevel {
  Error,
  Warning,
}

impl Diagnostic {
  pub fn new(message: String, level: DiagnosticLevel) -> Diagnostic {
    Diagnostic {
      level,
      message,
    }
  }

  pub fn error(message: String) -> Diagnostic {
    Diagnostic::new(message, DiagnosticLevel::Error)
  }

  pub fn warning(message: String) -> Diagnostic {
    Diagnostic::new(message, DiagnosticLevel::Warning)
  }
}
