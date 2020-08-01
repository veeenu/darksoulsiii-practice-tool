#include <windows.h>
#include <psapi.h>
#include <iostream>
#include <fstream>
#include <string>

static std::ofstream* LOG;

/*
bool rlower_equals(const std::string& a, const std::string& b) {
  auto it_a = a.rbegin();
  auto it_b = b.rbegin();
  while (it_a != a.rend() && it_b != b.rend()) {
    if (tolower(*it_a) != tolower(*it_b)) return false;
    ++it_a; ++it_b;
  }
  return true;
}

bool lower_equals(const std::string& a, const std::string& b) {
  auto it_a = a.begin();
  auto it_b = b.begin();
  while (it_a != a.end() && it_b != b.end()) {
    if (tolower(*it_a) != tolower(*it_b)) return false;
    ++it_a; ++it_b;
  }
  return true;
}
*/

DWORD find_process(const std::string& name) {
  DWORD ret_pid = -1;
  HMODULE ret_base = 0;
  MODULEINFO ret_info;

  DWORD lpidProcess[5120];
  unsigned long cbNeeded, count;
  HMODULE hModule[256];
  char modname[256];

  EnumProcesses(lpidProcess, sizeof(lpidProcess), &cbNeeded);
  int nReturned = cbNeeded / sizeof(cbNeeded);

  *LOG << nReturned << std::endl;
  for (int i = 0; i < nReturned; i++) {
    auto pid = lpidProcess[i];
    auto hProc = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid);
    if(!hProc) continue;

    EnumProcessModules(hProc, hModule, sizeof(hModule), &count);
    GetModuleBaseNameA(hProc, hModule[0], modname, sizeof(modname));

    *LOG << hProc << " " << hModule[0] << std::endl;
    std::string procname(modname);
    *LOG << "[*] Matching process " << i << ": " << procname << "...";
    LOG->flush();
    // if (rlower_equals(procname, name)) {
    if (procname == name) {
      GetModuleInformation(hProc, hModule[0], &ret_info, sizeof(ret_info));
      ret_pid = pid;
      ret_base = hModule[0];
      *LOG << " found!" << std::endl;
      break;
    } else {
      *LOG << " no" << std::endl;
    } 
    LOG->flush();

    for (int j = 0; j < 256; j++) modname[j] = 0;
    CloseHandle(hProc);
  }

  return ret_pid;
}

int APIENTRY WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nShowCmd) {
  LOG = new std::ofstream("jdsd_dsiii_practice_tool.log");

  char full_dll_path[_MAX_PATH];
  GetFullPathNameA("jdsd_dsiii_practice_tool.dll", _MAX_PATH, full_dll_path, NULL);
  *LOG << "[*] Full path name: " << full_dll_path << std::endl;

  DWORD pid = find_process("DarkSoulsIII.exe");
  HANDLE h_process = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid);
  *LOG << "[*] Process handle: " << h_process << " / " << pid << std::endl;

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
