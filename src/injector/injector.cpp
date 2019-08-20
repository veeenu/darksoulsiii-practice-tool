#include <windows.h>
#include <psapi.h>
#include <iostream>
#include <string>

bool lower_equals(const std::string& a, const std::string& b) {
  auto it_a = a.begin();
  auto it_b = b.begin();
  while (it_a != a.end() && it_b != b.end()) {
    if (tolower(*it_a) != tolower(*it_b)) return false;
    ++it_a; ++it_b;
  }
  return true;
}

DWORD find_process(const std::string& name) {
  DWORD ret_pid = -1;
  HMODULE ret_base = 0;
  MODULEINFO ret_info;

  DWORD lpidProcess[256];
  unsigned long cbNeeded, count;
  HMODULE hModule[64];
  char modname[30];

  EnumProcesses(lpidProcess, sizeof(lpidProcess), &cbNeeded);
  int nReturned = cbNeeded / sizeof(cbNeeded);

  for (int i = 0; i < nReturned; i++) {
    auto pid = lpidProcess[i];
    auto hProc = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid);
    if(!hProc) continue;

    EnumProcessModules(hProc, hModule, sizeof(hModule), &count);
    GetModuleBaseNameA(hProc, hModule[0], modname, sizeof(modname));

    std::string procname(modname);
    if (lower_equals(procname, name)) {
      GetModuleInformation(hProc, hModule[0], &ret_info, sizeof(ret_info));
      ret_pid = pid;
      ret_base = hModule[0];
    }

    for (int j = 0; j < 30; j++) modname[j] = 0;
    CloseHandle(hProc);
  }

  return ret_pid;
}

int APIENTRY WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nShowCmd) {
  char full_dll_path[_MAX_PATH];
  GetFullPathNameA("jdsd_dsiii_practice_tool.dll", _MAX_PATH, full_dll_path, NULL);

  DWORD pid = find_process("DarkSoulsIII.exe");
  HANDLE h_process = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid);

  void* proc_dll_path = VirtualAllocEx(h_process, NULL, _MAX_PATH, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);

  WriteProcessMemory(h_process, proc_dll_path, full_dll_path, _MAX_PATH, NULL);

  HANDLE h_thread = CreateRemoteThread(
    h_process, NULL, 0,    
    (LPTHREAD_START_ROUTINE) GetProcAddress(GetModuleHandle(__TEXT("Kernel32")), "LoadLibraryA"), 
    proc_dll_path, 0, NULL);

  WaitForSingleObject(h_thread, INFINITE);

  DWORD exit_code;
  GetExitCodeThread(h_thread, &exit_code);  
  CloseHandle(h_thread);
  VirtualFreeEx(h_process, proc_dll_path, 0, MEM_RELEASE); 
  CloseHandle(h_process);
}