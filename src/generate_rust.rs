use crate::{error::Error, schema};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

// This function generates Rust code from a schema and its transitive dependencies.
pub fn generate(
    _schemas: &HashMap<PathBuf, (schema::Schema, String)>,
    _rust_out_dir: &Path,
) -> Result<(), Error> {
    Ok(())
}
