
use std::path::Path;

fn inject(dll: &Path) {
    let path =
        widestring::WideCString::from_os_str(dll.canonicalize().unwrap().as_os_str()).unwrap();

    println!("{:?}", path);
}

fn main() {
    inject(Path::new("téest"));
}
