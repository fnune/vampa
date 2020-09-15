use crate::definitions::Compiler;

use vamc_parser::definitions::ast::SourceFile;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn compile_source_file(&mut self, source_file: SourceFile) {
        let target_block = self.context.append_basic_block(
            *self
                .function_value
                .expect("Expected a function value in the compiler but found None."),
            &source_file.file_name,
        );

        self.builder.position_at_end(target_block);

        source_file.statements.into_iter().for_each(|statement| {
            self.compile_statement(target_block, *statement)
                .expect("Failed to compile top-level statement.");
        });
    }
}
