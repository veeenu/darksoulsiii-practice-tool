use hudhook::prelude::inject;

fn main() {
  let mut cur_exe = std::env::current_exe().unwrap();
  cur_exe.push("..");
  cur_exe.push("libjdsd_dsiii_practice_tool.dll");

  let cur_dll = cur_exe.canonicalize().unwrap();

  inject("DarkSoulsIII.exe", cur_dll.as_path().to_str().unwrap()).unwrap();
}
