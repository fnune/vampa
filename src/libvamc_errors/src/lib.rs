use vamc_span::Span;

#[derive(Debug, PartialEq)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub span: Option<Span>,
}

#[derive(Debug, PartialEq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
}

impl Diagnostic {
    pub fn new(message: String, level: DiagnosticLevel) -> Diagnostic {
        Diagnostic {
            level,
            message,
            span: None,
        }
    }

    pub fn error(message: String) -> Diagnostic {
        Diagnostic::new(message, DiagnosticLevel::Error)
    }

    pub fn warning(message: String) -> Diagnostic {
        Diagnostic::new(message, DiagnosticLevel::Warning)
    }

    pub fn error_at(message: String, span: Span) -> Diagnostic {
        Diagnostic {
            level: DiagnosticLevel::Error,
            message,
            span: Some(span),
        }
    }
}
