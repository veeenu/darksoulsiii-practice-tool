#include "logging.h"
#include "ui.h"
#include "config.h"
#include "tinyformat.h"
#include <windows.h>

Command::Command(const callback& _fn, const std::string& _label, const uint64_t _key) :
  fn(_fn), label(_label), key(_key) {}

void Command::set_key(const uint64_t k) {
  key = k;
}

const uint64_t Command::get_key() const {
  return key;
}

const std::string& Command::get_label() const {
  return label;
}

void Command::operator() () {
  log() << "Executing command " << label << std::endl;
  fn();
}

std::unique_ptr<UI> UI::instance;
UI::UI () {
  prevKeys.reserve(512);

  for (short i = 0; i < 512; i++) {
    prevKeys[i] = 0;
  }

  auto cfg = Config::Instance();


  // Toggle
  commands.push_back(Command([this]() {
    show_window = !show_window;
  }, "show", cfg["show"]));

#define TOGGLE_CMD(NAME)            \
  commands.push_back(               \
    Command([this]() {              \
      if (auto p = state.toggle_##NAME()) { \
        NAME = *p;                  \
      }                             \
    }, #NAME, cfg[#NAME])           \
  );

  TOGGLE_CMD(no_damage)
  TOGGLE_CMD(no_death)
  TOGGLE_CMD(deathcam)
  TOGGLE_CMD(inf_stamina)
  TOGGLE_CMD(inf_focus)
  TOGGLE_CMD(inf_consum)
  TOGGLE_CMD(one_shot)
  TOGGLE_CMD(event_draw)
  TOGGLE_CMD(event_disable)
  TOGGLE_CMD(ai_disable)
  TOGGLE_CMD(no_gravity)
  TOGGLE_CMD(rend_chr)
  TOGGLE_CMD(rend_map)
  TOGGLE_CMD(rend_obj)

#undef TOGGLE_CMD

  commands.push_back(Command([this]() {
    // speed = state.cycle_speed();
    if (auto t = state.cycle_speed()) {
      speed = *t;
    }
  }, "cycle_speed", cfg["cycle_speed"]));

  commands.push_back(Command([this]() {
    // speed = state.cycle_speed();
    state.incr_souls();
  }, "incr_souls", cfg["incr_souls"]));

  commands.push_back(Command([this]() {
    state.save_pos();
  }, "save_pos", cfg["save_pos"]));

  commands.push_back(Command([this]() {
    state.load_pos();
  }, "load_pos", cfg["load_pos"]));

  commands.push_back(Command([this]() {
    state.quitout();
  }, "quitout", cfg["quitout"]));
}

UI& const UI::Instance () {
  if (!instance) {
    instance.reset(new UI());
  }
  return *(instance.get());
}

void UI::Render() {
  ImGuiIO& io = ImGui::GetIO();
  auto& cfg = Config::Instance();

  for (auto& i : commands) {
    if (is_keyup(io, i.get_key())) {
      i();
    }
  }

  constexpr auto window_flags =
      ImGuiWindowFlags_NoDecoration
    | ImGuiWindowFlags_NoCollapse
    | ImGuiWindowFlags_NoResize
    | ImGuiWindowFlags_NoMove
    | ImGuiWindowFlags_NoScrollbar
    ;

  if (show_window) {
    ReadMemory();

    ImGui::SetNextWindowBgAlpha(0.3f);
    ImGui::PushStyleVar(ImGuiStyleVar_WindowRounding, 0.0);
    ImGui::SetNextWindowPos(ImVec2(25., 100.));
    ImGui::SetNextWindowSize(ImVec2(320., 0.));
    if (ImGui::Begin("Practice tool", nullptr, window_flags)) {

      ImGui::Columns(2);
#define CHKBOX(LABEL, CODE) \
    { \
      ImGui::SetColumnWidth(0, 256); \
      ImGui::SetColumnWidth(1, 64); \
      ImGui::Checkbox(LABEL, &CODE); \
      ImGui::NextColumn(); \
      ImGui::Text(tfm::format("%s", cfg.repr(#CODE)).c_str()); \
      ImGui::NextColumn(); \
    }
    //ImGui::Checkbox(tfm::format(LABEL, cfg.repr(#CODE)).c_str(), &CODE);
      CHKBOX("No Damage", no_damage)
      CHKBOX("No Death", no_death)
      CHKBOX("Deathcam", deathcam)
      CHKBOX("Inf Stamina", inf_stamina)
      CHKBOX("Inf Focus", inf_focus)
      CHKBOX("Inf Consumables", inf_consum)
      CHKBOX("One Shot", one_shot)
      CHKBOX("Event Draw", event_draw)
      CHKBOX("Event Disable", event_disable)
      CHKBOX("AI Disable", ai_disable)
      CHKBOX("Gravity", no_gravity)
      CHKBOX("Render Character", rend_chr)
      CHKBOX("Render Map", rend_map)
      CHKBOX("Render Objects", rend_obj)
#undef CHKBOX
      if (auto pos = state.get_position()) {
        ImGui::Text(tfm::format("Position [saved]: \n  x % 12.5f [% 12.5f]\n  y % 12.5f [% 12.5f]\n  z % 12.5f [% 12.5f]", 
          std::get<0>(*pos), std::get<3>(*pos),
          std::get<1>(*pos), std::get<4>(*pos),
          std::get<2>(*pos), std::get<5>(*pos)
        ).c_str());
        ImGui::NextColumn();
        ImGui::Text(tfm::format("Load:\n% 5s\nSave:\n% 5s", cfg.repr("load_pos"), cfg.repr("save_pos")).c_str());
        ImGui::NextColumn();
      } else {
        ImGui::Text(tfm::format("Position [saved]: \n  x % 12.5f [% 12.5f]\n  y % 12.5f [% 12.5f]\n  z % 12.5f [% 12.5f]", 
          0., 0., 0., 0., 0., 0.
        ).c_str());
        ImGui::NextColumn();
        ImGui::Text(tfm::format("Load:\n% 5s\nSave:\n% 5s", cfg.repr("load_pos"), cfg.repr("save_pos")).c_str());
        ImGui::NextColumn();
      }
      ImGui::Text(tfm::format("Speed: [% 10.2f]", speed).c_str());
      ImGui::NextColumn();
      ImGui::Text(cfg.repr("cycle_speed").c_str());
      ImGui::NextColumn();
      ImGui::Text(tfm::format("Souls: [% 10d]", souls).c_str());
      ImGui::NextColumn();
      ImGui::Text(cfg.repr("incr_souls").c_str());
      ImGui::NextColumn();
      //ImGui::Text(tfm::format("Version %s | Quitout (%s)", state.get_version(), cfg.repr("quitout")).c_str());
      ImGui::Text("Version");
      ImGui::NextColumn();
      ImGui::Text(state.get_version().c_str());
      ImGui::NextColumn();
      ImGui::Text("Quitout");
      ImGui::NextColumn();
      ImGui::Text(cfg.repr("quitout").c_str());
      ImGui::NextColumn();
    }
    ImGui::PopStyleVar();
    ImGui::End();
  }

  for (int i = 0; i < 512; i++) {
    prevKeys[i] = io.KeysDown[i];
  }

}

// Attempt reading memory and refreshing values
void UI::ReadMemory() {
  #define READ_ONE(NAME) if (auto t = state.get_##NAME()) { NAME = *t; }
  READ_ONE(no_damage)
  READ_ONE(no_death)
  READ_ONE(deathcam)
  READ_ONE(inf_stamina)
  READ_ONE(inf_focus)
  READ_ONE(inf_consum)
  READ_ONE(one_shot)
  READ_ONE(event_draw)
  READ_ONE(event_disable)
  READ_ONE(ai_disable)
  READ_ONE(no_gravity)
  READ_ONE(rend_chr)
  READ_ONE(rend_map)
  READ_ONE(rend_obj)
  READ_ONE(souls)
}

bool UI::is_keyup(const ImGuiIO& io, int k) {
  return !io.KeysDown[k] && prevKeys[k];
}
