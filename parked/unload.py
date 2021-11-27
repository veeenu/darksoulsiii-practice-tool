from win32gui import *
from win32api import *
from win32process import *

PROCESS_ALL_ACCESS = 0x1F0FFF
hwnd = FindWindow(None, 'DARK SOULS III')
tid, pid = GetWindowThreadProcessId(hwnd)

hproc = OpenProcess(PROCESS_ALL_ACCESS, False, pid)
mods = EnumProcessModules(hproc)
modf = [ (mod, GetModuleFileNameEx(hproc, mod)) for mod in mods]

print(mods)
print(modf)

for mod, modname in modf:
    if modname.endswith('hook_you.dll'):
        print(modname)
        print(mod)

# let title = CString::new("DARK SOULS III").unwrap();
# let hwnd = unsafe { FindWindowA(null_mut(), title.as_ptr() as *const i8) };

# if hwnd == null_mut() {
#     error!("FindWindowA returned NULL: {}", unsafe { GetLastError() });
#     return;
# }

# let mut pid: DWORD = 0;
# unsafe { GetWindowThreadProcessId(hwnd, &mut pid as *mut _ as _) };
