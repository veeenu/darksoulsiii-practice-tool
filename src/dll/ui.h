#pragma once

#include <functional>
#include <vector>
#include <string>
#include <memory>
#include <imgui.h>
#include <tinyformat.h>

#include "memory.h"

typedef std::function<void(void)> callback;

class Command {
  private:
    callback fn;
    std::string label;
    uint64_t key;

  public:
    Command(const callback& _fn, const std::string& _label, uint64_t _key);
    void set_key(const uint64_t k);
    const uint64_t get_key() const;
    const std::string& get_label() const;
    void operator() ();
};

class UI {
  private:
    static std::unique_ptr<UI> instance;
    UI();

    std::vector<bool> prevKeys;
    std::vector<Command> commands;

    bool show_window = true;

    bool no_damage = false;
    bool no_death = false;
    bool deathcam = false;
    bool inf_stamina = false;
    bool inf_focus = false;
    bool inf_consum = false;
    bool one_shot = false;
    bool event_draw = false;
    bool event_disable = false;
    bool ai_disable = false;
    bool no_gravity = true;
    bool rend_chr = true;
    bool rend_map = true;
    bool rend_obj = true;

    float speed = 0.0f;

    MemoryState state;

  public:
    static UI& const Instance();
    void Render();
    void ReadMemory();
    bool is_keyup(const ImGuiIO& io, int k);
};
