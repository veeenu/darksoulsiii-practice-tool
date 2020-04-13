#include "memory.h"
#include <iostream>

/*template<typename T>
inline bool toggle_bit(T* ptr, uint8_t bit = 0) {
	T mask = (1 << bit);
	if (*ptr & mask) {
		*ptr = (*ptr) & ~mask;
		return false;
	} else {
		*ptr = (*ptr) | mask;
		return true;
	}
}*/

// Crash-safe version with ReadProcessMemory/WriteProcessMemory
template<typename T>
inline std::optional<bool> toggle_bit(std::optional<T*> ptr, uint8_t bit = 0) {  
	T mask = (1 << bit);
  T buf;
  bool ret;

  auto proc = GetCurrentProcess();

  std::cout << "toggle_bit(" << std::hex << (uint64_t)*ptr
            << ", " << std::hex << (uint64_t)bit << ")" << std::endl;

  if (
    (!ptr) ||
    *ptr == nullptr ||
    !ReadProcessMemory(proc, *ptr, &buf, sizeof(T), nullptr)
  ) {
    return {};
  }

	if (buf & mask) {
		buf = buf & ~mask;
		ret = false;
	} else {
		buf = buf | mask;
		ret = true;
	}

  WriteProcessMemory(
    proc,
    *ptr,
    &buf,
    sizeof(T),
    nullptr
  );

  return ret;
}

template<typename T>
inline std::optional<bool> get_bit(std::optional<T*> ptr, uint8_t bit = 0) {
	T mask = (1 << bit);
  T buf;
  if (
    (!ptr) ||
    *ptr == nullptr || 
    !ReadProcessMemory(GetCurrentProcess(), *ptr, &buf, sizeof(T), nullptr)
  ) {
    return {};
  } else {
    return (buf & mask);
  }
}

constexpr BaseAddresses base_addresses_104 {
	0x1404BC5FA, // base b
    0x88,      // deathcam
    0x1ECA,    // no goods consume
    0xA38,     // speed
	0x1404C1DC0, // base d
	0x1404C527D, // base f
	0x140830AF1, // xa
	0x1408C3388, // debug
		1,         // player exterminate
		9 + 4,     // no update ai
	0x140620B1B, // game rend
	0x1446A9280, // insta qo
  0x14288C422  // version string
};

constexpr BaseAddresses base_addresses_108 {
	0x1404C0DDA, // base b
    0x88,      // deathcam
    0x1EDA,    // no goods consume
    0xA38,     // speed
	0x1404C6580, // base d
	0x1404C9A4D, // base f
	0x14083BA91, // xa
	0x1408D06F8, // debug
		1,         // player exterminate
		9 + 3,     // no update ai
	0x1406287AB, // game rend
	0x1447103D8, // insta qo
  0x1428D3F92  // version string
};

constexpr BaseAddresses base_addresses_112 { // TODO
	0x1404C191A, // base b
    0x90,      // deathcam
    0x1EE2,    // no goods consume
    0xA58,     // speed
	0x1404C7120, // base d
	0x1404CA5ED, // base f
	0x140841875, // xa
	0x1408D7C88, // debug
		1,         // player exterminate
		9 + 4,     // no update ai
	0x14062C45B, // game rend
	0x144746988, // insta qo
  0x1428FD262  // version string
};

constexpr BaseAddresses base_addresses_115 {
	0x1404C1A3A, // base b
    0x90,      // deathcam
    0x1EEA,    // no goods consume
    0xA58,     // speed
	0x1404C7240, // base d
	0x1404CA70D, // base f
	0x140841D05, // xa
	0x1408D9748, // debug
		1,         // player exterminate
		9 + 4,     // no update ai
	0x14062C58B, // game rend
	// 0x141069F30, // speed
	0x14474C2E8, // insta qo
  0x142900782  // version string
};

MemoryState::MemoryState() {
	BaseAddresses b;

  // Rudimentary AF
  if (wcsncmp(reinterpret_cast<wchar_t*>(base_addresses_104.version_string_ptr), L"1.04", 4) == 0) {
    std::cout << "Detected version 1.04" << std::endl;
    version = std::string("1.04");
    b = base_addresses_104;
  } else if (wcsncmp(reinterpret_cast<wchar_t*>(base_addresses_108.version_string_ptr), L"1.08", 4) == 0) {
    std::cout << "Detected version 1.08" << std::endl;
    version = std::string("1.08");
    b = base_addresses_108;
  } else if (wcsncmp(reinterpret_cast<wchar_t*>(base_addresses_112.version_string_ptr), L"1.12", 4) == 0) {
    std::cout << "Detected version 1.12" << std::endl;
    version = std::string("1.12");
    b = base_addresses_112;
  } else if (wcsncmp(reinterpret_cast<wchar_t*>(base_addresses_115.version_string_ptr), L"1.15", 4) == 0) {
    std::cout << "Detected version 1.15" << std::endl;
    version = std::string("1.15");
    b = base_addresses_115;
  } else {
    std::cout << "Unrecognized version, stuff will crash, bye!" << std::endl;
    version = std::string("W.TF");
  }

	auto base_b = BASE_CHAIN(uint64_t, b.base_b, 3, 7);
	auto base_d = BASE_CHAIN(uint64_t, b.base_d, 3, 7);
	auto base_f = BASE_CHAIN(uint64_t, b.base_f, 3, 7);
	auto debug  = BASE_CHAIN(uint64_t, b.debug,  3, 7);
	auto grend  = BASE_CHAIN(uint64_t, b.grend,  2, 7);
	auto xa     = *(uint32_t*)(b.xa + 3);

	x = POINTER_CHAIN(float, base_b, 0x40, 0x28, 0x80);
	y = POINTER_CHAIN(float, base_b, 0x40, 0x28, 0x88);
	z = POINTER_CHAIN(float, base_b, 0x40, 0x28, 0x84);
	speed = POINTER_CHAIN(float, base_b, 0x80, xa, 0x28, b.offs_speed);
	// speed = reinterpret_cast<float*>(b.speed);
	
	p_rend_chr = POINTER_CHAIN(uint8_t, grend + 2);
	p_rend_map = POINTER_CHAIN(uint8_t, grend + 0);
	p_rend_obj = POINTER_CHAIN(uint8_t, grend + 1);
	p_oneshot = POINTER_CHAIN(uint8_t, debug + b.offs_player_exterminate);
	p_ai_disable = POINTER_CHAIN(uint8_t, debug + b.offs_no_update_ai);

	p_quitout = POINTER_CHAIN(uint8_t, b.instaqo, 0x250);
	p_deathcam = POINTER_CHAIN(uint8_t, base_b, b.offs_deathcam);
	p_evt_draw = POINTER_CHAIN(uint8_t, base_f, 0xA8);
	p_evt_disable = POINTER_CHAIN(uint8_t, base_f, 0xD4);
	p_flags = POINTER_CHAIN(uint8_t, base_b, 0x80, xa, 0x18, 0x1C0); // inf stam, inf focus, no death
	p_inf_consum = POINTER_CHAIN(uint8_t, base_b, 0x80, b.offs_no_goods_consume); // 1eea
	p_no_damage = POINTER_CHAIN(uint8_t, base_b, 0x80, 0x1A09);
	p_no_grav = POINTER_CHAIN(uint8_t, base_d, 0x60, 0x48);
}

void MemoryState::save_pos () {
  std::optional<float*> px, py, pz;
  if (
    (px = x()) &&
    (py = y()) &&
    (pz = z())
  ) {
    auto proc = GetCurrentProcess();
    ReadProcessMemory(proc, *px, &stored_x, sizeof(stored_x), nullptr);
    ReadProcessMemory(proc, *py, &stored_y, sizeof(stored_y), nullptr);
    ReadProcessMemory(proc, *pz, &stored_z, sizeof(stored_z), nullptr);
    /*stored_x = **px;
    stored_y = **py;
    stored_z = **pz;*/
  }
}

void MemoryState::load_pos () {
  std::optional<float*> px, py, pz;
  if (
    (px = x()) &&
    (py = y()) &&
    (pz = z())
  ) {
    auto proc = GetCurrentProcess();
    WriteProcessMemory(proc, *px, &stored_x, sizeof(stored_x), nullptr);
    WriteProcessMemory(proc, *py, &stored_y, sizeof(stored_y), nullptr);
    WriteProcessMemory(proc, *pz, &stored_z, sizeof(stored_z), nullptr);
    /***px = stored_x;
    **py = stored_y;
    **pz = stored_z;*/
  }
}

std::optional<float> MemoryState::cycle_speed() {
	cur_speed_idx = (cur_speed_idx + 1) % allowed_speeds.size();
	float s = allowed_speeds[cur_speed_idx];

  if (auto speed_ptr = speed()) {
    **speed_ptr = s;
    return s;
  } else {
    return {};
  }
}

std::optional<float> MemoryState::get_speed () { 
  if (auto speed_ptr = speed()) { 
    return **speed_ptr; 
  } else {
    return {};
  }
}

std::optional<bool> MemoryState::get_no_damage ()     { return get_bit(p_no_damage(), 7); }
std::optional<bool> MemoryState::get_no_death ()      { return get_bit(p_flags(), 2); }
std::optional<bool> MemoryState::get_deathcam ()      { return get_bit(p_deathcam()); }
std::optional<bool> MemoryState::get_inf_stamina ()   { return get_bit(p_flags(), 4); }
std::optional<bool> MemoryState::get_inf_focus ()     { return get_bit(p_flags(), 5); }
std::optional<bool> MemoryState::get_inf_consum ()    { return get_bit(p_inf_consum(), 3); }
std::optional<bool> MemoryState::get_one_shot ()      { return get_bit(p_oneshot()); }
std::optional<bool> MemoryState::get_event_draw ()    { return get_bit(p_evt_draw()); }
std::optional<bool> MemoryState::get_event_disable () { return get_bit(p_evt_disable()); }
std::optional<bool> MemoryState::get_ai_disable ()    { return get_bit(p_ai_disable()); }
std::optional<bool> MemoryState::get_no_gravity ()    { return get_bit(p_no_grav()); }
std::optional<bool> MemoryState::get_rend_chr ()      { return get_bit(p_rend_chr()); }
std::optional<bool> MemoryState::get_rend_map ()      { return get_bit(p_rend_map()); }
std::optional<bool> MemoryState::get_rend_obj ()      { return get_bit(p_rend_obj()); }

std::optional<bool> MemoryState::toggle_no_damage ()     { return toggle_bit(p_no_damage(), 7); }
std::optional<bool> MemoryState::toggle_no_death ()      { return toggle_bit(p_flags(), 2); }
std::optional<bool> MemoryState::toggle_deathcam ()      { return toggle_bit(p_deathcam()); }
std::optional<bool> MemoryState::toggle_inf_stamina ()   { return toggle_bit(p_flags(), 4); }
std::optional<bool> MemoryState::toggle_inf_focus ()     { return toggle_bit(p_flags(), 5); }
std::optional<bool> MemoryState::toggle_inf_consum ()    { return toggle_bit(p_inf_consum(), 3); }
std::optional<bool> MemoryState::toggle_one_shot ()      { return toggle_bit(p_oneshot()); }
std::optional<bool> MemoryState::toggle_event_draw ()    { return toggle_bit(p_evt_draw()); }
std::optional<bool> MemoryState::toggle_event_disable () { return toggle_bit(p_evt_disable()); }
std::optional<bool> MemoryState::toggle_ai_disable ()    { return toggle_bit(p_ai_disable()); }
std::optional<bool> MemoryState::toggle_no_gravity ()    { return toggle_bit(p_no_grav()); }
std::optional<bool> MemoryState::toggle_rend_chr ()      { return toggle_bit(p_rend_chr()); }
std::optional<bool> MemoryState::toggle_rend_map ()      { return toggle_bit(p_rend_map()); }
std::optional<bool> MemoryState::toggle_rend_obj ()      { return toggle_bit(p_rend_obj()); }

const std::string& MemoryState::get_version () const {
  return version;
}

void MemoryState::quitout () {
  if (auto ptr = p_quitout()) {
    **ptr = 1;
  }
}

std::optional<std::tuple<float, float, float, float, float, float>> MemoryState::get_position() const {
	std::optional<float*> px; //= x();
  std::optional<float*> py; //= y();
  std::optional<float*> pz; //= z();

  if ((px = x()) && (py = y()) && (pz = z())) {
    float vx, vy, vz;
    auto proc = GetCurrentProcess();
    if (
      ReadProcessMemory(proc, *px, &vx, sizeof(vx), nullptr) &&
      ReadProcessMemory(proc, *py, &vy, sizeof(vy), nullptr) &&
      ReadProcessMemory(proc, *pz, &vz, sizeof(vz), nullptr)
    ) {
      return std::make_optional(std::make_tuple(
        vx,
        vy,
        vz,
        stored_x, stored_y, stored_z
      ));
    } else {
      return {};
    }
  } else {
    return {};
  }
};