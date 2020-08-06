#include <windows.h>
#include <psapi.h>
#include <iostream>
#include <fstream>
#include <string>
#include <optional>

static std::ofstream* LOG;

std::optional<DWORD> find_process(const std::string& name) {
  HMODULE ret_base = 0;
  MODULEINFO ret_info;

  DWORD lpidProcess[10240];
  unsigned long cbNeeded, count;
  HMODULE hModule[256];
  char modname[256];

  EnumProcesses(lpidProcess, sizeof(lpidProcess), &cbNeeded);
  int nReturned = cbNeeded / sizeof(cbNeeded);

  for (int i = 0; i < nReturned; i++) {
    auto pid = lpidProcess[i];
    auto hProc = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid);
    if(!hProc) continue;

    EnumProcessModules(hProc, hModule, sizeof(hModule), &count);
    GetModuleBaseNameA(hProc, hModule[0], modname, sizeof(modname));

    std::string procname(modname);
    if (procname == name) {
      GetModuleInformation(hProc, hModule[0], &ret_info, sizeof(ret_info));
      // ret_pid = pid;
      ret_base = hModule[0];
      CloseHandle(hProc);
      return pid;
      break;
    } 

    for (int j = 0; j < 256; j++) modname[j] = 0;
  }

  *LOG << "[!] Couldn't find process." << std::endl;
  return {};

}

int APIENTRY WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nShowCmd) {
  LOG = new std::ofstream("jdsd_dsiii_practice_tool.log", std::fstream::out | std::fstream::app);

  *LOG << "[*] Searching for process DarkSoulsIII.exe..." << std::endl;

  char full_dll_path[_MAX_PATH];
  GetFullPathNameA("jdsd_dsiii_practice_tool.dll", _MAX_PATH, full_dll_path, NULL);
  *LOG << "[*] Full DLL path name: " << full_dll_path << std::endl;

  std::optional<DWORD> pid = find_process("DarkSoulsIII.exe");

  if (!pid) {
    LOG->flush();
    LOG->close();
    return -1;
  }

  HANDLE h_process = OpenProcess(PROCESS_ALL_ACCESS, FALSE, *pid);
  *LOG << "[*] Process handle: " << h_process << " / " << *pid << std::endl;

  void* proc_dll_path = VirtualAllocEx(h_process, NULL, _MAX_PATH, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);

  WriteProcessMemory(h_process, proc_dll_path, full_dll_path, _MAX_PATH, NULL);

  HANDLE h_thread = CreateRemoteThread(
    h_process, NULL, 0,    
    (LPTHREAD_START_ROUTINE) GetProcAddress(GetModuleHandle(__TEXT("Kernel32")), "LoadLibraryA"), 
    proc_dll_path, 0, NULL);
  *LOG << "[*] Thread: " << h_thread << std::endl;

  WaitForSingleObject(h_thread, INFINITE);
  *LOG << "[*] Finished waiting" << std::endl;

  DWORD exit_code;
  GetExitCodeThread(h_thread, &exit_code);  
  CloseHandle(h_thread);
  VirtualFreeEx(h_process, proc_dll_path, 0, MEM_RELEASE); 
  CloseHandle(h_process);
  LOG->flush();
  LOG->close();
}
