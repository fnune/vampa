use std::path::PathBuf;

use vamc_backend::Backend;
use vamc_lexer::{cursor::Cursor, definitions::Token};
use vamc_llvm::LlvmBackend;
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

        let tokens: Vec<Token> = Cursor::new(&source).collect();
        let mut parser = Parser::new(tokens);
        let program = parser
            .parse_source_file(source_path)
            .expect("Failed to parse source file.");

        let object_bytes = LlvmBackend
            .compile_program(program)
            .map_err(|diagnostic| format!("{diagnostic:?}"))?;

        let object_path = PathBuf::from(source_path).with_extension("o");
        let object_str = object_path
            .to_str()
            .expect("Object path is not valid UTF-8.");

        self.host
            .write_object(object_str, &object_bytes)
            .map_err(|error| format!("Failed to write {object_str}: {error}"))?;

        let executable_path = PathBuf::from(source_path).with_extension("");
        let executable_str = executable_path
            .to_str()
            .expect("Executable path is not valid UTF-8.");

        self.host
            .link(object_str, executable_str)
            .map_err(|error| format!("Failed to link {executable_str}: {error}"))?;

        Ok(executable_path)
    }
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
        linked: Rc<RefCell<Option<(String, String)>>>,
    }

    impl Host for FakeHost {
        fn read_source(&self, _path: &str) -> io::Result<String> {
            Ok(self.source.clone())
        }

        fn write_object(&self, path: &str, bytes: &[u8]) -> io::Result<()> {
            *self.written.borrow_mut() = Some((path.to_string(), bytes.to_vec()));
            Ok(())
        }

        fn link(&self, object_path: &str, executable_path: &str) -> io::Result<()> {
            *self.linked.borrow_mut() =
                Some((object_path.to_string(), executable_path.to_string()));
            Ok(())
        }
    }

    #[test]
    fn compiles_in_memory_through_a_substituted_host() {
        let written = Rc::new(RefCell::new(None));
        let linked = Rc::new(RefCell::new(None));
        let host = FakeHost {
            source: "fun three returning i32 = 3;\napply three".to_string(),
            written: written.clone(),
            linked: linked.clone(),
        };

        let output = Driver::new(host).compile("mem.vam").unwrap();
        assert_eq!(output, PathBuf::from("mem"));

        let recorded = written.borrow();
        let (object_path, bytes) = recorded.as_ref().unwrap();
        assert_eq!(object_path, "mem.o");
        assert_eq!(&bytes[0..4], b"\x7fELF");

        let linked = linked.borrow();
        assert_eq!(
            linked.as_ref().unwrap(),
            &("mem.o".to_string(), "mem".to_string())
        );
    }
}
