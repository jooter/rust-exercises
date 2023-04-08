use std::fs;

fn main() -> std::io::Result<()> {
    // Copy the file from the source path to the destination path
    let source_path = "source_file.txt";
    let dest_path = "destination_file.txt";
    fs::copy(source_path, dest_path)?;
    Ok(())
}
