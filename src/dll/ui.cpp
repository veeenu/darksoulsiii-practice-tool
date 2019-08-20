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
  std::cout << "Executing command " << label << std::endl;
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

#define TOGGLE_CMD(NAME) commands.push_back(Command([this]() { NAME = state.toggle_##NAME(); }, #NAME, cfg[#NAME]));

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
    speed = state.cycle_speed();
  }, "cycle_speed", cfg["cycle_speed"]));

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
    /*
#define REFRESH(FLAG) FLAG = state.get_##FLAG();
    REFRESH(no_damage);
    REFRESH(no_death);
    REFRESH(deathcam);
    REFRESH(inf_stamina);
    REFRESH(inf_focus);
    REFRESH(inf_consum);
    REFRESH(one_shot);
    REFRESH(event_draw);
    REFRESH(event_disable);
    REFRESH(ai_disable);
    REFRESH(no_gravity);
    REFRESH(rend_chr);
    REFRESH(rend_map);
    REFRESH(rend_obj);
    REFRESH(speed);
#undef REFRESH
    */

    ImGui::SetNextWindowBgAlpha(0.3f);
    ImGui::PushStyleVar(ImGuiStyleVar_WindowRounding, 0.0);
    if (ImGui::Begin("Practice tool", nullptr, window_flags)) {

      ImGui::SetWindowPos(ImVec2(25., 100.));
#define CHKBOX(LABEL, CODE) ImGui::Checkbox(tfm::format(LABEL, cfg.repr(#CODE)).c_str(), &CODE);
      CHKBOX("No Damage (%s)", no_damage)
      CHKBOX("No Death (%s)", no_death)
      CHKBOX("Deathcam (%s)", deathcam)
      CHKBOX("Inf Stamina (%s)", inf_stamina)
      CHKBOX("Inf Focus (%s)", inf_focus)
      CHKBOX("Inf Consumables (%s)", inf_consum)
      CHKBOX("One Shot (%s)", one_shot)
      CHKBOX("Event Draw (%s)", event_draw)
      CHKBOX("Event Disable (%s)", event_disable)
      CHKBOX("AI Disable (%s)", ai_disable)
      CHKBOX("Gravity (%s)", no_gravity)
      CHKBOX("Render Character (%s)", rend_chr)
      CHKBOX("Render Map (%s)", rend_map)
      CHKBOX("Render Objects (%s)", rend_obj)
#undef CHKBOX
      auto pos = state.get_position(); 
      ImGui::Text(tfm::format("Position [saved]: \n  x % 12.5f [% 12.5f]\n  y % 12.5f [% 12.5f]\n  z % 12.5f [% 12.5f]\n  (Load %s | Save %s)", 
        std::get<0>(pos), std::get<3>(pos),
        std::get<1>(pos), std::get<4>(pos),
        std::get<2>(pos), std::get<5>(pos),
        cfg.repr("load_pos"), cfg.repr("save_pos")
      ).c_str());
      ImGui::Text(tfm::format("Speed: % 3.2f", speed).c_str());
      ImGui::Text(tfm::format("Version %s | Quitout (%s)", state.get_version(), cfg.repr("quitout")).c_str());
    }
    ImGui::PopStyleVar();
    ImGui::End();
  }

  for (int i = 0; i < 512; i++) {
    prevKeys[i] = io.KeysDown[i];
  }

}

bool UI::is_keyup(const ImGuiIO& io, int k) {
  return !io.KeysDown[k] && prevKeys[k];
}
