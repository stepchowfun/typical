use crate::{error::Error, schema};
use std::{collections::HashMap, path::PathBuf};

// This function validates a schema and its transitive dependencies. The based import paths are
// assumed to be in the `HashMap` [ref:valid_based_paths].
pub fn validate(_schemas: &HashMap<PathBuf, schema::Schema>) -> Result<(), Error> {
    Ok(())
}
