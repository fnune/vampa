use vamc_errors::Diagnostic;
use vamc_parser::definitions::ast::SourceFile;

pub trait Backend {
    fn compile_program(&self, program: SourceFile) -> Result<Vec<u8>, Diagnostic>;
}
