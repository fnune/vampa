#[cfg(test)]
mod tests;

#[cfg(test)]
mod behavior;

pub mod definitions;

mod llvm_backend;
pub use llvm_backend::LlvmBackend;

// All the things that can be compiled.
mod binary_operation;
mod block;
mod expression;
mod function_call;
mod function_declaration;
mod literal;
mod parameters;
mod source_file;
mod statement;
mod typ;
mod variable_declaration;
mod variable_reference;
