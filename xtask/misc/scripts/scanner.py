import time
import numpy as np
import ctypes
from ctypes import *
from ctypes.wintypes import *
from itertools import cycle

k32 = windll.kernel32
psapi = windll.psapi
SIZE_T = c_ulong

VirtualAllocEx = k32.VirtualAllocEx
VirtualFreeEx = k32.VirtualFreeEx
ReadProcessMemory = k32.ReadProcessMemory
WriteProcessMemory = k32.WriteProcessMemory
CreateRemoteThreadEx = k32.CreateRemoteThreadEx
OpenProcess = k32.OpenProcess
CloseHandle = k32.CloseHandle
EnumProcesses = psapi.EnumProcesses
EnumProcessModules = psapi.EnumProcessModules
GetModuleBaseNameA = psapi.GetModuleBaseNameA
GetModuleInformation = psapi.GetModuleInformation
GetLastError = k32.GetLastError

VirtualAllocEx.restype = LPVOID
VirtualAllocEx.argtypes = (HANDLE, LPVOID, DWORD, DWORD, DWORD)
VirtualFreeEx.argtypes = (HANDLE, LPVOID, SIZE_T, DWORD)
ReadProcessMemory.argtypes = (
    HANDLE, LPCVOID, LPVOID, SIZE_T, POINTER(c_ulong)
)
WriteProcessMemory.argtypes = (HANDLE, LPVOID, LPCVOID, DWORD, LPDWORD)
CreateRemoteThreadEx.argtypes = (
    HANDLE, LPVOID, DWORD, LPVOID, LPVOID, DWORD, LPVOID, LPDWORD
)


class MODULEINFORMATION(Structure):
    _fields_ = [
        ("lpBaseOfDll", LPVOID),
        ("SizeOfImage", DWORD),
        ("EntryPoint", LPVOID)
    ]


# Constants
DEBUG_PROCESS = 0x00000001
CREATE_NEW_CONSOLE = 0x00000010
PROCESS_ALL_ACCESS = 0x001F0FFF
INFINITE = 0xFFFFFFFF
DBG_CONTINUE = 0x00010002


# Debug event constants
EXCEPTION_DEBUG_EVENT = 0x1
CREATE_THREAD_DEBUG_EVENT = 0x2
CREATE_PROCESS_DEBUG_EVENT = 0x3
EXIT_THREAD_DEBUG_EVENT = 0x4
EXIT_PROCESS_DEBUG_EVENT = 0x5
LOAD_DLL_DEBUG_EVENT = 0x6
UNLOAD_DLL_DEBUG_EVENT = 0x7
OUTPUT_DEBUG_STRING_EVENT = 0x8
RIP_EVENT = 0x9

# debug exception codes.
EXCEPTION_ACCESS_VIOLATION = 0xC0000005
EXCEPTION_BREAKPOINT = 0x80000003
EXCEPTION_GUARD_PAGE = 0x80000001
EXCEPTION_SINGLE_STEP = 0x80000004


# Thread constants for CreateToolhelp32Snapshot()
TH32CS_SNAPHEAPLIST = 0x00000001
TH32CS_SNAPPROCESS = 0x00000002
TH32CS_SNAPTHREAD = 0x00000004
TH32CS_SNAPMODULE = 0x00000008
TH32CS_INHERIT = 0x80000000
TH32CS_SNAPALL = (TH32CS_SNAPHEAPLIST | TH32CS_SNAPPROCESS |
                  TH32CS_SNAPTHREAD | TH32CS_SNAPMODULE)
THREAD_ALL_ACCESS = 0x001F03FF

# Context flags for GetThreadContext()
CONTEXT_FULL = 0x00010007
CONTEXT_DEBUG_REGISTERS = 0x00010010

# Memory permissions
PAGE_EXECUTE_READWRITE = 0x00000040

# Hardware breakpoint conditions
HW_ACCESS = 0x00000003
HW_EXECUTE = 0x00000000
HW_WRITE = 0x00000001

# Memory page permissions, used by VirtualProtect()
PAGE_NOACCESS = 0x00000001
PAGE_READONLY = 0x00000002
PAGE_READWRITE = 0x00000004
PAGE_WRITECOPY = 0x00000008
PAGE_EXECUTE = 0x00000010
PAGE_EXECUTE_READ = 0x00000020
PAGE_EXECUTE_READWRITE = 0x00000040
PAGE_EXECUTE_WRITECOPY = 0x00000080
PAGE_GUARD = 0x00000100
PAGE_NOCACHE = 0x00000200
PAGE_WRITECOMBINE = 0x00000400


def find_process(name):
    arr = c_ulong * 2560
    lpidProcess = arr()
    cb = sizeof(lpidProcess)
    cbNeeded = c_ulong()
    hModule = (HMODULE * 256)()
    count = c_ulong()
    modname = create_string_buffer(30)
    PROCESS_QUERY_INFORMATION = 0x0400
    PROCESS_VM_READ = 0x0010

    EnumProcesses(byref(lpidProcess),
                  cb,
                  byref(cbNeeded))

    nReturned = int(cbNeeded.value / sizeof(c_ulong()))

    pidProcess = [i for i in lpidProcess][:nReturned]

    ret = (None, None)

    for pid in pidProcess:

        hProcess = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                               False, pid)
        if hProcess:
            EnumProcessModules(hProcess,
                               byref(hModule),
                               sizeof(hModule),
                               byref(count))
            GetModuleBaseNameA(hProcess,
                               c_ulonglong(hModule[0]),
                               modname,
                               sizeof(modname))
            procname = ''.join([str(i.decode('utf-8'))
                                for i in modname if i != b'\x00'])

            if procname.lower() == name.lower():
                hModInfo = MODULEINFORMATION()
                GetModuleInformation(hProcess,
                                     c_ulonglong(hModule[0]),
                                     byref(hModInfo),
                                     sizeof(MODULEINFORMATION))
                ret = {'name': procname, 'pid': pid,
                       'base': hModule[0], 'info': hModInfo}

            for i in range(modname._length_):
                modname[i] = 0

            CloseHandle(hProcess)

    return ret


def readb(h_proc, addr, ctype=c_uint64):
    buf = ctype()
    bw = c_ulong()
    r = ReadProcessMemory(h_proc, addr, byref(buf), sizeof(buf), byref(bw))
    if r != 1:
        print(f'ReadProcessMemory: {r}: {GetLastError():x}')
    return buf


def read_chain(h_proc, offsets, ctype=c_uint64):
    addr = 0
    for offs in offsets[:-1]:
        addr = readb(h_proc, offs + addr).value
    val = readb(h_proc, addr + offsets[-1], ctype)
    return val


def writeb(h_proc, addr, value):
    bw = c_ulong()
    r = WriteProcessMemory(h_proc, addr, byref(value),
                           sizeof(value), byref(bw))
    print(f'WriteProcessMemory @ {addr:x} -> {bytes(value).hex()}: {r}')


def write_chain(h_proc, offsets, value):
    addr = 0
    for offs in offsets[:-1]:
        addr = readb(h_proc, offs + addr).value
    writeb(h_proc, addr + offsets[-1], value)


if __name__ == '__main__':
    proc = find_process('DarkSoulsIII.exe')
    h_proc = OpenProcess(PROCESS_ALL_ACCESS, False, proc['pid'])
    mods = EnumProcessModules(h_proc)

    
    def calc(base_b, base_d, base_f, debug, grend, xa):
        boffs = readb(h_proc, base_b + 3, c_uint32).value
        val = base_b + 7 + boffs
        print(f'base_b: 0x{val:x}')
        boffs = readb(h_proc, base_d + 3, c_uint32).value
        val = base_d + 7 + boffs
        print(f'base_d: 0x{val:x}')
        boffs = readb(h_proc, base_f + 3, c_uint32).value
        val = base_f + 7 + boffs
        print(f'base_f: 0x{val:x}')
        boffs = readb(h_proc, debug + 3, c_uint32).value
        val = debug + 7 + boffs
        print(f'debug: 0x{val:x}')
        boffs = readb(h_proc, grend + 2, c_uint32).value
        val = grend + 7 + boffs
        print(f'grend: 0x{val:x}')
        boffs = readb(h_proc, xa + 3, c_uint32).value
        print(f'xa: 0x{boffs:x}')

    # 1.04
    # calc(0x1404BC5FA, 0x1404C1DC0, 0x1404C527D, 0x1408C3388, 0x140620B1B, 0x140830AF1)
    # 1.08
    # calc(0x1404C0DDA, 0x1404C6580, 0x1404C9A4D, 0x1408D06F8, 0x1406287AB, 0x14083BA91)
    # 1.12
    # calc(0x1404C191A,0x1404C7120, 0x1404CA5ED, 0x1408D7C88, 0x14062C45B, 0x140841875)
    # 1.15
    calc(0x1404C1A3A, 0x1404C7240,0x1404CA70D, 0x1408D9748, 0x14062C58B,0x140841D05)  

