use std::collections::HashMap;

use inkwell::OptimizationLevel;
use inkwell::context::Context;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use vamc_backend::Backend;
use vamc_errors::Diagnostic;
use vamc_parser::definitions::ast::SourceFile;

use crate::definitions::Compiler;

pub struct LlvmBackend;

impl Backend for LlvmBackend {
    fn compile_program(&self, program: SourceFile) -> Result<Vec<u8>, Diagnostic> {
        let context = Context::create();
        let module = context.create_module(&program.file_name);
        let builder = context.create_builder();

        let mut compiler = Compiler {
            context: &context,
            builder: &builder,
            module: &module,
            function_value: None,
            variables: HashMap::new(),
        };

        let function_type = context.i32_type().fn_type(&[], false);
        let function_value = module.add_function("main", function_type, None);
        compiler.function_value = Some(&function_value);

        compiler.compile_source_file(program);

        Target::initialize_native(&InitializationConfig::default()).map_err(|error| {
            Diagnostic::error(format!("Failed to initialize the native target: {error}"))
        })?;

        let triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&triple).map_err(|error| {
            Diagnostic::error(format!("Failed to look up the native target: {error}"))
        })?;
        let machine = target
            .create_target_machine(
                &triple,
                TargetMachine::get_host_cpu_name().to_str().unwrap(),
                TargetMachine::get_host_cpu_features().to_str().unwrap(),
                OptimizationLevel::None,
                RelocMode::PIC,
                CodeModel::Default,
            )
            .ok_or_else(|| Diagnostic::error("Failed to create a target machine.".into()))?;

        module.set_triple(&triple);
        module.set_data_layout(&machine.get_target_data().get_data_layout());

        let buffer = machine
            .write_to_memory_buffer(&module, FileType::Object)
            .map_err(|error| {
                Diagnostic::error(format!("Failed to emit a native object file: {error}"))
            })?;

        Ok(buffer.as_slice().to_vec())
    }
}
