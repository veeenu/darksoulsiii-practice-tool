#pragma once

#include <unordered_map>
#include <string>
#include <windows.h>
#include <cpptoml.h>

std::string get_config_file_name ();

class Config {
  private:
    static std::unique_ptr<Config> instance;
    Config();

    std::unordered_map<std::string, DWORD> mappings;

    std::unordered_map<std::string, DWORD> hotkey_string_mappings;
    std::unordered_map<DWORD, std::string> string_hotkey_mappings;

    std::unordered_map<std::string, std::string> settings;

    void save ();
    void load ();

  public:
    static Config& Instance();

    DWORD operator[] (std::string i) const;
    std::string setting (const std::string& i) const;
    const std::string& repr (std::string i);
};