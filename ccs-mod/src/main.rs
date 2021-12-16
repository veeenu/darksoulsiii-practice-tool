use hudhook::inject;

fn main() {
    let mut dll_path = std::env::current_exe().unwrap();
    dll_path.pop();
    dll_path.push("ccs_mod.dll");
    inject::inject("DARK SOULS III", dll_path);
}
