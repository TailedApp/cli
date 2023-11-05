use std::{io, fs, env};
use std::path::Path;

fn main() {
    let src_path = Path::new("./src/rules/sets");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir)
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap()
        .join("rules").join("sets");

    println!("src: {}", src_path.to_string_lossy().to_owned());
    println!("dst: {}", dest_path.to_string_lossy().to_owned());
    
    let _ = fs::remove_dir_all(&dest_path);
    copy_dir_all(&src_path, &dest_path).unwrap();

    // Uncomment to view println statements.
    //panic!();
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}