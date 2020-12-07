use crate::error::Error;
use std::{collections::HashMap, fs, path::Path};

/// Read User Environment into a HashMap. Returns an empty HashMap if the path does not exist.
pub fn read_env(path: impl AsRef<Path>) -> Result<HashMap<String, String>, Error> {
    let path = path.as_ref();
    let mut env = HashMap::new();

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            let key_os = match path.file_stem() {
                Some(key_os) => key_os,
                None => continue,
            };
            if let Some(key) = key_os.to_str() {
                let value = fs::read_to_string(&path)?;
                env.insert(key.to_string(), value);
            }
        }
    }

    Ok(env)
}
