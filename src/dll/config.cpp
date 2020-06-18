#include "config.h"
#include <fstream>
#include <iostream>
#include <Shlwapi.h>

std::unique_ptr<Config> Config::instance;
std::vector<std::string> allowed_nodes {
  "no_damage", "no_death", "deathcam",
  "inf_stamina", "inf_focus", "inf_consum",
  "one_shot", "event_draw", "event_disable",
  "ai_disable", "no_gravity", "rend_chr",
  "rend_map", "rend_obj", "incr_souls",
  "show", "quitout", "save_pos", "load_pos"
};
std::vector<std::string> allowed_settings {
  "enabled", "debug"
};

std::string get_config_file_name () {
  HMODULE hModule;
  GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, "get_config_file_name", &hModule);
  char szFileName[MAX_PATH]; 
  GetModuleFileNameA(hModule, szFileName, MAX_PATH);
  std::string wf(szFileName);
  std::cout << wf << std::endl;
  uint64_t lastslash = wf.find_last_of('\\');
  std::string dirname(wf.begin(), wf.begin() + lastslash + 1);
  return dirname + std::string("jdsd_dsiii_practice_tool.toml");
}

Config& Config::Instance () {
  if (!instance) {
    instance.reset(new Config());
  }

  return *instance.get();
}

Config::Config () {

  for(auto& i : std::unordered_map<std::string, DWORD> ({
    { "VK_LBUTTON", VK_LBUTTON },
    { "VK_RBUTTON", VK_RBUTTON },
    { "VK_CANCEL", VK_CANCEL },
    { "VK_MBUTTON", VK_MBUTTON },
    { "VK_XBUTTON1", VK_XBUTTON1 },
    { "VK_XBUTTON2", VK_XBUTTON2 },
    { "VK_BACK", VK_BACK },
    { "VK_TAB", VK_TAB },
    { "VK_CLEAR", VK_CLEAR },
    { "VK_RETURN", VK_RETURN },
    { "VK_SHIFT", VK_SHIFT },
    { "VK_CONTROL", VK_CONTROL },
    { "VK_MENU", VK_MENU },
    { "VK_PAUSE", VK_PAUSE },
    { "VK_CAPITAL", VK_CAPITAL },
    { "VK_KANA", VK_KANA },
    { "VK_HANGUL", VK_HANGUL },
    { "VK_JUNJA", VK_JUNJA },
    { "VK_FINAL", VK_FINAL },
    { "VK_HANJA", VK_HANJA },
    { "VK_KANJI", VK_KANJI },
    { "VK_ESCAPE", VK_ESCAPE },
    { "VK_CONVERT", VK_CONVERT },
    { "VK_NONCONVERT", VK_NONCONVERT },
    { "VK_ACCEPT", VK_ACCEPT },
    { "VK_MODECHANGE", VK_MODECHANGE },
    { "VK_SPACE", VK_SPACE },
    { "VK_PRIOR", VK_PRIOR },
    { "VK_NEXT", VK_NEXT },
    { "VK_END", VK_END },
    { "VK_HOME", VK_HOME },
    { "VK_LEFT", VK_LEFT },
    { "VK_UP", VK_UP },
    { "VK_RIGHT", VK_RIGHT },
    { "VK_DOWN", VK_DOWN },
    { "VK_SELECT", VK_SELECT },
    { "VK_PRINT", VK_PRINT },
    { "VK_EXECUTE", VK_EXECUTE },
    { "VK_SNAPSHOT", VK_SNAPSHOT },
    { "VK_INSERT", VK_INSERT },
    { "VK_DELETE", VK_DELETE },
    { "VK_HELP", VK_HELP },
    { "0", '0' },
    { "1", '1' },
    { "2", '2' },
    { "3", '3' },
    { "4", '4' },
    { "5", '5' },
    { "6", '6' },
    { "7", '7' },
    { "8", '8' },
    { "9", '9' },
    { "A", 'A' },
    { "B", 'B' },
    { "C", 'C' },
    { "D", 'D' },
    { "E", 'E' },
    { "F", 'F' },
    { "G", 'G' },
    { "H", 'H' },
    { "I", 'I' },
    { "J", 'J' },
    { "K", 'K' },
    { "L", 'L' },
    { "M", 'M' },
    { "N", 'N' },
    { "O", 'O' },
    { "P", 'P' },
    { "Q", 'Q' },
    { "R", 'R' },
    { "S", 'S' },
    { "T", 'T' },
    { "U", 'U' },
    { "V", 'V' },
    { "W", 'W' },
    { "X", 'X' },
    { "Y", 'Y' },
    { "Z", 'Z' },
    { "VK_LWIN", VK_LWIN },
    { "VK_RWIN", VK_RWIN },
    { "VK_APPS", VK_APPS },
    { "VK_SLEEP", VK_SLEEP },
    { "VK_NUMPAD0", VK_NUMPAD0 },
    { "VK_NUMPAD1", VK_NUMPAD1 },
    { "VK_NUMPAD2", VK_NUMPAD2 },
    { "VK_NUMPAD3", VK_NUMPAD3 },
    { "VK_NUMPAD4", VK_NUMPAD4 },
    { "VK_NUMPAD5", VK_NUMPAD5 },
    { "VK_NUMPAD6", VK_NUMPAD6 },
    { "VK_NUMPAD7", VK_NUMPAD7 },
    { "VK_NUMPAD8", VK_NUMPAD8 },
    { "VK_NUMPAD9", VK_NUMPAD9 },
    { "VK_MULTIPLY", VK_MULTIPLY },
    { "VK_ADD", VK_ADD },
    { "VK_SEPARATOR", VK_SEPARATOR },
    { "VK_SUBTRACT", VK_SUBTRACT },
    { "VK_DECIMAL", VK_DECIMAL },
    { "VK_DIVIDE", VK_DIVIDE },
    { "VK_F1", VK_F1 },
    { "VK_F2", VK_F2 },
    { "VK_F3", VK_F3 },
    { "VK_F4", VK_F4 },
    { "VK_F5", VK_F5 },
    { "VK_F6", VK_F6 },
    { "VK_F7", VK_F7 },
    { "VK_F8", VK_F8 },
    { "VK_F9", VK_F9 },
    { "VK_F10", VK_F10 },
    { "VK_F11", VK_F11 },
    { "VK_F12", VK_F12 },
    { "VK_F13", VK_F13 },
    { "VK_F14", VK_F14 },
    { "VK_F15", VK_F15 },
    { "VK_F16", VK_F16 },
    { "VK_F17", VK_F17 },
    { "VK_F18", VK_F18 },
    { "VK_F19", VK_F19 },
    { "VK_F20", VK_F20 },
    { "VK_F21", VK_F21 },
    { "VK_F22", VK_F22 },
    { "VK_F23", VK_F23 },
    { "VK_F24", VK_F24 },
    { "VK_NUMLOCK", VK_NUMLOCK },
    { "VK_SCROLL", VK_SCROLL },
    { "VK_LSHIFT", VK_LSHIFT },
    { "VK_RSHIFT", VK_RSHIFT },
    { "VK_LCONTROL", VK_LCONTROL },
    { "VK_RCONTROL", VK_RCONTROL },
    { "VK_LMENU", VK_LMENU },
    { "VK_RMENU", VK_RMENU },
    { "VK_BROWSER_BACK", VK_BROWSER_BACK },
    { "VK_BROWSER_FORWARD", VK_BROWSER_FORWARD },
    { "VK_BROWSER_REFRESH", VK_BROWSER_REFRESH },
    { "VK_BROWSER_STOP", VK_BROWSER_STOP },
    { "VK_BROWSER_SEARCH", VK_BROWSER_SEARCH },
    { "VK_BROWSER_FAVORITES", VK_BROWSER_FAVORITES },
    { "VK_BROWSER_HOME", VK_BROWSER_HOME },
    { "VK_VOLUME_MUTE", VK_VOLUME_MUTE },
    { "VK_VOLUME_DOWN", VK_VOLUME_DOWN },
    { "VK_VOLUME_UP", VK_VOLUME_UP },
    { "VK_MEDIA_NEXT_TRACK", VK_MEDIA_NEXT_TRACK },
    { "VK_MEDIA_PREV_TRACK", VK_MEDIA_PREV_TRACK },
    { "VK_MEDIA_STOP", VK_MEDIA_STOP },
    { "VK_MEDIA_PLAY_PAUSE", VK_MEDIA_PLAY_PAUSE },
    { "VK_LAUNCH_MAIL", VK_LAUNCH_MAIL },
    { "VK_LAUNCH_MEDIA_SELECT", VK_LAUNCH_MEDIA_SELECT },
    { "VK_LAUNCH_APP1", VK_LAUNCH_APP1 },
    { "VK_LAUNCH_APP2", VK_LAUNCH_APP2 },
    { "VK_OEM_1", VK_OEM_1 },
    { "VK_OEM_PLUS", VK_OEM_PLUS },
    { "VK_OEM_COMMA", VK_OEM_COMMA },
    { "VK_OEM_MINUS", VK_OEM_MINUS },
    { "VK_OEM_PERIOD", VK_OEM_PERIOD },
    { "VK_OEM_2", VK_OEM_2 },
    { "VK_OEM_3", VK_OEM_3 },
    { "VK_OEM_4", VK_OEM_4 },
    { "VK_OEM_5", VK_OEM_5 },
    { "VK_OEM_6", VK_OEM_6 },
    { "VK_OEM_7", VK_OEM_7 },
    { "VK_OEM_8", VK_OEM_8 },
    { "VK_OEM_102", VK_OEM_102 },
    { "VK_PROCESSKEY", VK_PROCESSKEY },
    { "VK_PACKET", VK_PACKET },
    { "VK_ATTN", VK_ATTN },
    { "VK_CRSEL", VK_CRSEL },
    { "VK_EXSEL", VK_EXSEL },
    { "VK_EREOF", VK_EREOF },
    { "VK_PLAY", VK_PLAY },
    { "VK_ZOOM", VK_ZOOM },
    { "VK_NONAME", VK_NONAME },
    { "VK_PA1", VK_PA1 },
    { "VK_OEM_CLEAR", VK_OEM_CLEAR }
  })) {
    hotkey_string_mappings.emplace(i.first, i.second);
    string_hotkey_mappings.emplace(i.second, i.first);
  }

#define EMPM(S, VK) mappings.emplace(#S, VK)
  EMPM(show, VK_F11);
  EMPM(quitout, 'P');
  EMPM(save_pos, VK_F7);
  EMPM(load_pos, VK_F1);
  EMPM(inf_stamina, VK_F2);
  EMPM(inf_focus, VK_F3);
  EMPM(inf_consum, VK_F4);
  EMPM(no_damage, VK_F5);
  EMPM(no_death, VK_F6);
  EMPM(deathcam, VK_F8);
  EMPM(one_shot, VK_F9);
  EMPM(no_gravity, VK_F10);
  EMPM(cycle_speed, '4');
  EMPM(event_draw, '5');
  EMPM(event_disable, '6');
  EMPM(ai_disable, '7');
  EMPM(rend_chr, '8');
  EMPM(rend_map, '9');
  EMPM(rend_obj, '0');
#undef EMPM

  settings.emplace("enabled", "true");
  settings.emplace("debug", "false");

  auto conf_name = get_config_file_name();
  if (!PathFileExistsA(conf_name.c_str())) {
    save();
  } else {
    load();
  }
}

void Config::save () {
  auto tbl = cpptoml::make_table();

  auto mappings_tbl = cpptoml::make_table();
  for (auto& i : mappings) {
    mappings_tbl->insert(i.first, string_hotkey_mappings.at(i.second));
  }

  auto settings_tbl = cpptoml::make_table();
  for (auto& i : settings) {
    settings_tbl->insert(i.first, i.second);
  }

  tbl->insert("mappings", mappings_tbl);
  tbl->insert("settings", settings_tbl);

  std::ofstream f (get_config_file_name());

  f << (*tbl) << std::endl;
}

void Config::load () {
  auto root = cpptoml::parse_file(get_config_file_name());

  auto mappings_node = root->get_table("mappings");
  auto settings_node = root->get_table("settings");

  for (auto& i : allowed_nodes) {
    auto v = mappings_node->get_qualified_as<std::string>(i);
    if (!v) continue;
    auto& f = hotkey_string_mappings.find(*v);
    if (f != hotkey_string_mappings.end()) {
      mappings[i] = (*f).second;
      std::cout << "Mapping " << i << " -> " << (*f).first << std::endl;
    } else {
      std::cout << "Keycode " << *v << " for command " << i << " not found! Using default" << std::endl;
    }
  }

  for (auto& i : allowed_settings) {
    auto v = settings_node->get_qualified_as<std::string>(i);
    if (!v) continue;
    std::cout << "Setting " <<  i << " -> " << (*v) << std::endl;
    settings[i] = *v;
    auto& r = settings.find(i);
    std::cout << "Check: " << (*r).second << std::endl;
  }
}

DWORD Config::operator[] (std::string i) const {
  auto& r = mappings.find(i);

  // TODO error handling is nonexistent here but not really important for now as
  // all the lookups are guaranteed having a default
  /* if (r == mappings.end()) {
    throw;
  }*/

  return (*r).second;
}

std::string Config::setting (const std::string& i)  const{
  auto& r = settings.find(i);

  return (*r).second;
}

const std::string& Config::repr (std::string i) {
  return (*(string_hotkey_mappings.find((*this)[i]))).second;
}