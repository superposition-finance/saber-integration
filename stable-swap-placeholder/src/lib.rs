pub mod processor; // program logic

#[cfg(not(feature = "exclude_entrypoint"))]
pub mod entrypoint; // main function