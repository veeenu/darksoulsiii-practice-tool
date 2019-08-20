#pragma once

#include <memory>
#include <vector>

#define P(type, addr) reinterpret_cast<type*>(addr)
#define POINTER_CHAIN(T, ...) PointerChain<T>(std::vector<uint64_t> { __VA_ARGS__ })
#define BASE_CHAIN(T, B, SA, SB) static_cast<T>(B + (uint32_t)(*(uint32_t*)(B + SA)) + SB)

template<typename T>
class PointerChain {
  private:
    std::vector<uint64_t> chain;
  public:
    PointerChain() = default;
    PointerChain(const PointerChain&) = default;
    PointerChain& operator=(const PointerChain&) = default;

    PointerChain& operator+(const PointerChain& rhs) {
      std::vector<T> pasted;
      pasted.reserve(chain.size() + rhs.chain.size());
      pasted.insert(pasted.end(), chain.begin(), chain.end());
      pasted.insert(pasted.end(), rhs.chain.begin(), rhs.chain.end());
      return PointerChain(pasted);
    }

    PointerChain(std::vector<uint64_t> _chain) : chain(_chain) {};
    T* operator() () const {
      uint64_t* p = reinterpret_cast<uint64_t*>(chain[0]);
      if ((void*)p == nullptr) return nullptr;
      for (int i = 1; i < chain.size(); i++) {
        if ((void*)*p == nullptr) return nullptr;
        p = (uint64_t*)(*p + chain[i]);
      }
      return reinterpret_cast<T*>(p);
    };
};

typedef struct {
  uint64_t base_b;
    uint32_t offs_deathcam;
    uint32_t offs_no_goods_consume;
    uint32_t offs_speed;
  uint64_t base_d;
  uint64_t base_f;
  uint64_t xa;
  uint64_t debug;
    uint32_t offs_player_exterminate;
    uint32_t offs_no_update_ai;
  uint64_t grend;
  // uint64_t speed;
  uint64_t instaqo;
  uint64_t version_string_ptr;
} BaseAddresses;

class MemoryState {
  private:
    static std::unique_ptr<MemoryState> instance;

    std::vector<float> allowed_speeds { 0.25, 0.5, 1, 1.5, 2 };
    uint8_t cur_speed_idx = 2;

    PointerChain<float> x, y, z, speed;
    PointerChain<uint8_t>
      p_rend_chr, p_rend_map, p_rend_obj, p_oneshot, p_ai_disable, // no-chain bytes
      p_quitout, p_deathcam, p_evt_draw, p_evt_disable, // bytes
      p_flags, p_inf_consum, p_no_damage, p_no_grav;  // flags

    float stored_x = 0.0f;
    float stored_y = 0.0f;
    float stored_z = 0.0f;

    std::string version;

  public:
    MemoryState ();

    void save_pos ();
    void load_pos ();
    float cycle_speed ();

    float get_speed ();

    bool get_no_damage ();
    bool get_no_death ();
    bool get_deathcam ();
    bool get_inf_stamina ();
    bool get_inf_focus ();
    bool get_inf_consum ();
    bool get_one_shot ();
    bool get_event_draw ();
    bool get_event_disable ();
    bool get_ai_disable ();
    bool get_no_gravity ();
    bool get_rend_chr ();
    bool get_rend_map ();
    bool get_rend_obj ();

    bool toggle_no_damage ();
    bool toggle_no_death ();
    bool toggle_deathcam ();
    bool toggle_inf_stamina ();
    bool toggle_inf_focus ();
    bool toggle_inf_consum ();
    bool toggle_one_shot ();
    bool toggle_event_draw ();
    bool toggle_event_disable ();
    bool toggle_ai_disable ();
    bool toggle_no_gravity ();
    bool toggle_rend_chr ();
    bool toggle_rend_map ();
    bool toggle_rend_obj ();
    void quitout();

    const std::string& get_version () const;

    std::tuple<float, float, float, float, float, float> get_position() const;
};

#undef P