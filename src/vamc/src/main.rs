use std::{
    collections::HashMap,
    env, fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

use inkwell::context::Context;
use vamc_lexer::{cursor::Cursor, definitions::Token};
use vamc_llvm::definitions::Compiler;
use vamc_parser::definitions::Parser;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let file_name = arguments.get(1).expect("Usage: vamc SOURCE_FILE");
    let contents = fs::read_to_string(file_name).expect("Failed to read file.");

    let tokens: Vec<Token> = Cursor::new(&contents).collect();
    let mut parser = Parser::new(tokens);
    let source_file_ast = parser.parse_source_file(file_name).unwrap();

    let context = Context::create();
    let module = context.create_module(file_name);
    let builder = context.create_builder();

    let mut compiler = Compiler {
        context: &context,
        builder: &builder,
        module: &module,
        function_value: None,
        variables: HashMap::new(),
    };

    let function_type = compiler.context.i32_type().fn_type(&[], false);
    let function_value = compiler.module.add_function("main", function_type, None);

    compiler.function_value = Some(&function_value);

    let mut output_path_buf = PathBuf::from(file_name);
    output_path_buf.set_extension("o");

    let output_path = Path::new(output_path_buf.to_str().unwrap());

    compiler.compile_source_file(source_file_ast);
    compiler.module.write_bitcode_to_path(output_path);

    fs::File::open(output_path)
        .unwrap()
        .set_permissions(fs::Permissions::from_mode(0o770))
        .unwrap();
}
