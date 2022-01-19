import time
import numpy as np
import ctypes
from ctypes import *
from ctypes.wintypes import *
import win32gui, win32api, win32process
from itertools import cycle

k32 = windll.kernel32
# psapi = windll.psapi
# SIZE_T = c_ulong
# 
# VirtualAllocEx = k32.VirtualAllocEx
# VirtualFreeEx = k32.VirtualFreeEx
ReadProcessMemory = k32.ReadProcessMemory
WriteProcessMemory = k32.WriteProcessMemory
# CreateRemoteThreadEx = k32.CreateRemoteThreadEx
# OpenProcess = k32.OpenProcess
# CloseHandle = k32.CloseHandle
# EnumProcesses = psapi.EnumProcesses
# EnumProcessModules = psapi.EnumProcessModules
# GetModuleBaseNameA = psapi.GetModuleBaseNameA
# GetModuleInformation = psapi.GetModuleInformation
# GetLastError = k32.GetLastError
# 
# VirtualAllocEx.restype = LPVOID
# VirtualAllocEx.argtypes = (HANDLE, LPVOID, DWORD, DWORD, DWORD)
# VirtualFreeEx.argtypes = (HANDLE, LPVOID, SIZE_T, DWORD)
ReadProcessMemory.argtypes = (
    HANDLE, LPCVOID, LPVOID, c_ulong, POINTER(c_ulong)
)
WriteProcessMemory.argtypes = (HANDLE, LPVOID, LPCVOID, DWORD, LPDWORD)
# CreateRemoteThreadEx.argtypes = (
#     HANDLE, LPVOID, DWORD, LPVOID, LPVOID, DWORD, LPVOID, LPDWORD
# )
# 
# 
# # Constants
# DEBUG_PROCESS = 0x00000001
# CREATE_NEW_CONSOLE = 0x00000010
PROCESS_ALL_ACCESS = 0x001F0FFF
# INFINITE = 0xFFFFFFFF
# DBG_CONTINUE = 0x00010002
# 
# 
# # Debug event constants
# EXCEPTION_DEBUG_EVENT = 0x1
# CREATE_THREAD_DEBUG_EVENT = 0x2
# CREATE_PROCESS_DEBUG_EVENT = 0x3
# EXIT_THREAD_DEBUG_EVENT = 0x4
# EXIT_PROCESS_DEBUG_EVENT = 0x5
# LOAD_DLL_DEBUG_EVENT = 0x6
# UNLOAD_DLL_DEBUG_EVENT = 0x7
# OUTPUT_DEBUG_STRING_EVENT = 0x8
# RIP_EVENT = 0x9
# 
# # debug exception codes.
# EXCEPTION_ACCESS_VIOLATION = 0xC0000005
# EXCEPTION_BREAKPOINT = 0x80000003
# EXCEPTION_GUARD_PAGE = 0x80000001
# EXCEPTION_SINGLE_STEP = 0x80000004
# 
# # Memory permissions
# PAGE_EXECUTE_READWRITE = 0x00000040


class Process:
    def __init__(self, name):
        self.name = name

    def __enter__(self):
        hwnd = win32gui.FindWindow(None, self.name)
        pid = win32process.GetWindowThreadProcessId(hwnd)
        self.handle = win32api.OpenProcess(PROCESS_ALL_ACCESS, 0, pid[1])
        print(self.handle)
        return self

    def __exit__(self, a, b, c):
        win32api.CloseHandle(self.handle)

    def read(self, addr, ctype=c_uint64):
        buf = ctype()
        bw = c_ulong()
        r = ReadProcessMemory(int(self.handle), addr, byref(buf), sizeof(buf), byref(bw))
        if r != 1:
            print(f'ReadProcessMemory: {r}: {GetLastError():x}')
            return None
        return buf

    def write(self, addr, value):
        bw = c_ulong()
        r = WriteProcessMemory(int(self.handle), addr, byref(value),
                               sizeof(value), byref(bw))
        print(f'WriteProcessMemory @ {addr:x} -> {bytes(value).hex()}: {r}')

    def read_chain(self, offsets, ctype=c_uint64):
        addr = 0
        for offs in offsets[:-1]:
            addr = self.read(offs + addr).value
        val = self.read(addr + offsets[-1], ctype)
        return val

    def write_chain(self, offsets, value):
        addr = 0
        for offs in offsets[:-1]:
            addr = self.read(offs + addr).value
        self.write(addr + offsets[-1], value)

IGT = [0x144704268, 0xa4]
POS = [0x14472CF58, 0x40, 0x28, 0x80]

if __name__ == '__main__':
    with Process("DARK SOULS III") as proc, open('data.txt', 'w') as fp:
        fp.write('igt,x,y,z\n')
        igts = set()
        while True:
            try:
                igt = proc.read_chain(IGT, ctype=c_uint64)
                pos = proc.read_chain(POS, ctype=c_float * 3)
                if igt is not None and pos is not None and igt.value not in igts:
                    igts.add(igt.value)
                    x, y, z = pos
                    fp.write(f'{igt.value},{x},{y},{z}\n')
            except KeyboardInterrupt:
                break
