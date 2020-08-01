#include "logging.h"
#include <chrono>
#include <Shlwapi.h>

std::ofstream& Logger::logstream() {
  return logfile;
}

std::ofstream& log() {
  return Logger::instance().logstream();
}

std::string get_logging_file_name () {
  HMODULE hModule;
  GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, "get_config_file_name", &hModule);
  char szFileName[MAX_PATH]; 
  GetModuleFileNameA(hModule, szFileName, MAX_PATH);
  std::string wf(szFileName);
  // Logger::instance().logstream() << wf << std::endl;
  uint64_t lastslash = wf.find_last_of('\\');
  std::string dirname(wf.begin(), wf.begin() + lastslash + 1);
  return dirname + std::string("jdsd_dsiii_practice_tool.log");
}

Logger* Logger::logger;
Logger& Logger::instance() {
  if (((void*)logger) == nullptr) {
    logger = new Logger();
  }

  return *logger;
}

Logger::Logger() {
  logfile.open(get_logging_file_name(), std::fstream::out | std::fstream::app);
  auto time = std::chrono::system_clock::to_time_t(std::chrono::system_clock::now());

  logfile << std::endl << "[*] Started logging at " << time << std::endl;
}

