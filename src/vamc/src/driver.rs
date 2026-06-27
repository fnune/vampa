use std::collections::HashMap;
use std::path::PathBuf;

use inkwell::context::Context;
use vamc_lexer::{cursor::Cursor, definitions::Token};
use vamc_llvm::definitions::Compiler;
use vamc_parser::definitions::Parser;

use crate::host::Host;

pub struct Driver<H: Host> {
    host: H,
}

impl<H: Host> Driver<H> {
    pub fn new(host: H) -> Self {
        Driver { host }
    }

    pub fn compile(&self, source_path: &str) -> Result<PathBuf, String> {
        let source = self
            .host
            .read_source(source_path)
            .map_err(|error| format!("Failed to read {source_path}: {error}"))?;

        let object_bytes = emit_object(source_path, &source);

        let mut output_path = PathBuf::from(source_path);
        output_path.set_extension("o");
        let output_str = output_path
            .to_str()
            .expect("Output path is not valid UTF-8.");

        self.host
            .write_object(output_str, &object_bytes)
            .map_err(|error| format!("Failed to write {output_str}: {error}"))?;

        Ok(output_path)
    }
}

fn emit_object(module_name: &str, source: &str) -> Vec<u8> {
    let tokens: Vec<Token> = Cursor::new(source).collect();
    let mut parser = Parser::new(tokens);
    let source_file_ast = parser
        .parse_source_file(module_name)
        .expect("Failed to parse source file.");

    let context = Context::create();
    let module = context.create_module(module_name);
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

    compiler.compile_source_file(source_file_ast);

    let buffer = compiler.module.write_bitcode_to_memory();
    let bytes = buffer.as_slice();

    // Bitcode is a stream of 32-bit words, but inkwell's buffer carries a
    // trailing terminator byte that pushes the length past a word boundary and
    // makes the file unreadable, so keep only whole words.
    let whole_words = bytes.len() - (bytes.len() % 4);
    bytes[..whole_words].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::Host;
    use std::cell::RefCell;
    use std::io;
    use std::rc::Rc;

    struct FakeHost {
        source: String,
        written: Rc<RefCell<Option<(String, Vec<u8>)>>>,
    }

    impl Host for FakeHost {
        fn read_source(&self, _path: &str) -> io::Result<String> {
            Ok(self.source.clone())
        }

        fn write_object(&self, path: &str, bytes: &[u8]) -> io::Result<()> {
            *self.written.borrow_mut() = Some((path.to_string(), bytes.to_vec()));
            Ok(())
        }
    }

    #[test]
    fn compiles_in_memory_through_a_substituted_host() {
        let written = Rc::new(RefCell::new(None));
        let host = FakeHost {
            source: "fun three returning i32 = 3;\napply three".to_string(),
            written: written.clone(),
        };

        let output = Driver::new(host).compile("mem.vam").unwrap();
        assert_eq!(output, PathBuf::from("mem.o"));

        let recorded = written.borrow();
        let (path, bytes) = recorded.as_ref().unwrap();
        assert_eq!(path, "mem.o");
        assert_eq!(&bytes[0..2], b"BC");
    }
}
