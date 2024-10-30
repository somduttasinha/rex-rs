use std::{collections::HashMap, error, fs, path::PathBuf};

pub(crate) fn create_new_document(
    doc_type: &str,
    doc_name: &str,
    relative_dir: &str,
) -> Result<(), Box<dyn error::Error>> {
    let rex_root = std::env::var("REX_ROOT").expect("Expected REX_ROOT to be set.");

    let mut doc_types_available = HashMap::<String, PathBuf>::new();

    for entry in fs::read_dir(format!("{}/templates", rex_root))? {
        let entry = entry?;

        let path = entry.path();

        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("tex") {
            doc_types_available.insert(
                path.file_stem()
                    .expect("Expected file name")
                    .to_string_lossy()
                    .to_string(),
                path,
            );
        }
    }

    if !doc_types_available.contains_key(doc_type) {
        eprintln!("Error!");
        return Ok(());
    }

    println!("Ye");

    let to = std::env::current_dir()?
        .join(relative_dir);

    println!("{:?}", &to);

    let to = fs::canonicalize(to)?;

    let to = to.join(format!("{}.tex", doc_name));

    let from = match doc_types_available.get(doc_type) {
        Some(path) => path,
        None => {
            eprintln!("No matching document type found");
            return Ok(());
        }
    };

    println!("Copying to {}", &to.to_string_lossy());

    let _ = fs::copy(from, to);

    Ok(())
}
