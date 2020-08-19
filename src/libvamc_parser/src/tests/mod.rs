#[cfg(test)]
mod blocks {
    mod blocks;
}

#[cfg(test)]
mod variables {
    mod variable_declarations;
    mod variable_reference;
}

#[cfg(test)]
mod functions {
    mod function_calls;
    mod functions_explicit_return_type;
    mod functions_implicit_return_type;
    mod functions_with_block_body;
}

#[cfg(test)]
mod expressions {
    mod binary_operations;
}

#[cfg(test)]
mod modules {
    mod single_modules;
}
