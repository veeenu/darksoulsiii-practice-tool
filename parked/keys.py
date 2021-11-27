from ctypes import *
from ctypes.wintypes import *

user32 = WinDLL('user32', use_last_error = True)

GetKeyNameText = user32.GetKeyNameTextW
GetKeyNameText.argtypes = [c_long, LPWSTR, c_int]
GetKeyNameText.restype = c_int

buf = create_unicode_buffer(32)
for i in range(128):
    GetKeyNameText(c_long(i << 16), buf, 32)
    print(i, buf.value)
