#pragma once

#include <iostream>
#include <fstream>

class Logger {
  private:
    Logger();
    Logger(const Logger&) = delete;
    Logger(Logger&&) = delete;
    Logger& operator=(const Logger&) = delete;
    Logger& operator=(Logger&&) = delete;

    std::ofstream logfile;
    static Logger* logger;
  public:
    static Logger& instance();
    std::ofstream& logstream();
};

std::ofstream& log();
