use hudhook::*;

use std::sync::atomic::{AtomicUsize, Ordering};

use winapi::um::errhandlingapi;
use winapi::um::memoryapi;
use winapi::um::processthreadsapi;
use winapi::um::synchapi;

use dynasmrt::{dynasm, DynasmApi};
use imgui::*;
use log::*;

use super::item_ids::{ITEM_IDS, INFUSION_TYPES, UPGRADES};
use super::{Command, BUTTON_HEIGHT, BUTTON_WIDTH};
use crate::config::get_symbol;

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub(crate) struct ItemSpawn {
  label: imgui::ImString,
  instance_id: usize,
  hotkey_spawn: Option<i32>,
  item_id: u32,
  infusion: u32,
  upgrade: u32,
  qty: u32,
  durability: u32,
  addrs: (u64, u64, u64),

  item_id_idx: usize,
  infusion_idx: usize,
  upgrade_idx: usize,
}

impl ItemSpawn {
  pub(crate) fn new(
    label: &str,
    item_id: u32,
    infusion: u32,
    upgrade: u32,
    qty: u32,
    durability: u32,
    function_ptr: u64,
    unk_addr1: u64,
    unk_addr2: u64,
    hotkey_spawn: Option<i32>,
  ) -> ItemSpawn {
    ItemSpawn {
      label: imgui::ImString::new(label.to_string()),
      instance_id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
      hotkey_spawn,
      item_id,
      infusion,
      upgrade,
      qty,
      durability,
      addrs: (function_ptr, unk_addr1, unk_addr2),
      item_id_idx: ITEM_IDS
        .iter()
        .position(|(id, _)| id == &Some(item_id))
        .unwrap_or(0),
      infusion_idx: INFUSION_TYPES
        .iter()
        .position(|(id, _)| *id == infusion)
        .unwrap_or(0),
      upgrade_idx: UPGRADES
        .iter()
        .position(|(id, _)| *id == upgrade)
        .unwrap_or(0),
    }
  }

  pub(crate) fn is_valid(&self) -> bool {
    // self.pointer.eval().is_some()
    true
  }

  fn spawn(&self) {
    let mut ops = dynasmrt::x64::Assembler::new().unwrap();
    let item_id = self.item_id + self.infusion + self.upgrade;
    dynasm!(ops
    ; .arch x64
    ; sub rsp, 0x48
    ; lea r8d, [rsp + 0x20]
    ; lea rdx, [rsp + 0x30]
    ; mov eax, DWORD self.qty as _
    ; mov ebx, DWORD item_id as _
    ; mov esi, DWORD self.durability as _
    ; mov DWORD [rsp + 0x30], 1u32 as _
    ; mov [rsp + 0x3c], esi
    ; mov [rsp + 0x34], ebx
    ; mov [rsp + 0x38], eax
    ; mov rax, QWORD self.addrs.1 as i64
    ; mov rax, [rax]
    ; mov rbx, rax
    ; mov rax, QWORD self.addrs.2 as i64
    ; mov rax, [rax]
    ; mov rbp, [rax + 0x80]
    ; mov rcx, rbx
    ; mov rsi, QWORD self.addrs.0 as _
    ; call rsi
    ; add rsp, BYTE 0x48
    ; ret
    );

    std::thread::spawn(move || {
      let buf = ops.finalize().unwrap();
      let bufp: &[u8] = &buf;
      debug!(
        "Buffer: {}",
        (0..105)
          .into_iter()
          .map(|i| format!("{:02x}", bufp[i]))
          .collect::<Vec<_>>()
          .join(" ")
      );
      let hproc = unsafe { processthreadsapi::GetCurrentProcess() };
      let addr = unsafe {
        memoryapi::VirtualAllocEx(
          hproc,
          std::ptr::null_mut(),
          buf.size(),
          0x1000 | 0x2000, // MEM_COMMIT | MEM_RESERVE
          0x40,            // PAGE_EXECUTE_READWRITE
        )
      };

      if addr == std::ptr::null_mut() {
        error!("VirtualAllocEx: {:x}", unsafe {
          errhandlingapi::GetLastError()
        });
        return;
      }

      let mut bw = 0usize;
      let ret = unsafe {
        memoryapi::WriteProcessMemory(
          hproc,
          addr,
          std::mem::transmute(bufp.as_ptr()),
          buf.size(),
          &mut bw,
        )
      };

      if ret == 0 {
        error!("WriteProcessMemory: {:x}", unsafe {
          errhandlingapi::GetLastError()
        });
        return;
      }

      let thread = unsafe {
        processthreadsapi::CreateRemoteThreadEx(
          hproc,
          std::ptr::null_mut(),
          256,
          Some(std::mem::transmute(addr)),
          std::ptr::null_mut(),
          0,
          std::ptr::null_mut(),
          std::ptr::null_mut(),
        )
      };

      if thread == std::ptr::null_mut() {
        error!("CreateRemoteThreadEx: {:x}", unsafe {
          errhandlingapi::GetLastError()
        });
        return;
      }

      unsafe { synchapi::WaitForSingleObject(thread, 0xffffffff) };

      let ret = unsafe { memoryapi::VirtualFreeEx(hproc, addr, 0, 0x8000) };

      if ret == 0 {
        error!("VirtualFreeEx: {:x}", unsafe {
          errhandlingapi::GetLastError()
        });
        return;
      }
    });
  }

  // fn filter(&mut self) {}

  // fn filtered_combo_box(&mut self, ui: &imgui::Ui) {
  //   if ui
  //     .input_text(im_str!("##filter"), &mut self.input_text)
  //     .build()
  //   {
  //     self.filter();
  //   }

  //   let mut pos = ui.item_rect_min();
  //   let mut size = ui.item_rect_size();

  //   ui.same_line(0.);
  //   if ui.arrow_button(im_str!("##openCombo"), imgui::Direction::Down) {
  //     ui.open_popup(im_str!("combobox"));
  //   }

  //   pos[1] += size[1];
  //   size[0] += ui.item_rect_size()[0];
  //   size[1] += 5. + (size[1] * 20.);

  //   unsafe {
  //     imgui::sys::igSetNextWindowPos(
  //       imgui::sys::ImVec2::new(pos[0], pos[1]),
  //       imgui::sys::ImGuiCond_Always as _,
  //       imgui::sys::ImVec2::new(0., 0.),
  //     );
  //     imgui::sys::igSetNextWindowSize(
  //       imgui::sys::ImVec2::new(size[0], size[1]),
  //       imgui::sys::ImGuiCond_Always as _,
  //     );
  //   }
  //   ui.popup(im_str!("combobox"), || {
  //     for (idx, (item, label)) in ITEM_IDS.iter().enumerate() {
  //       let selected = idx == self.item_id_idx;
  //       if Selectable::new(&label).selected(selected).build(ui) {
  //         self.item_id_idx = idx;
  //         if let Some(item) = item {
  //           self.item_id = *item;
  //           self.input_text.clear();
  //           self.input_text.push_str(label.to_str());
  //           ui.close_current_popup();
  //           break;
  //         }
  //       }
  //     }
  //   });
  // }
}

impl Command for ItemSpawn {
  fn display(&mut self, ui: &imgui::Ui) -> bool {
    let mut size = ui.window_size();

    let id_tok = ui.push_id(self.instance_id as i32);

    let width = size[0] * 0.45;
    let w_tok = ui.push_item_width(width);

    let preview = &ITEM_IDS.get(self.item_id_idx).unwrap_or(&ITEM_IDS[0]).1;
    let combo = ComboBox::new(im_str!("##item_spawn"))
      .preview_value(preview)
      .height(ComboBoxHeight::Large);
    combo.build(ui, || {
      for (idx, (item, label)) in ITEM_IDS.iter().enumerate() {
        let selected = idx == self.item_id_idx;
        if Selectable::new(&label).selected(selected).build(ui) {
          self.item_id_idx = idx;
          if let Some(item) = item {
            self.item_id = *item;
          }
        }
      }
    });

    ui.same_line(0.);
    if ui.button(&self.label, [width, BUTTON_HEIGHT]) {
      self.spawn();
    }

    let preview = &INFUSION_TYPES.get(self.infusion_idx).unwrap_or(&INFUSION_TYPES[0]).1;
    let combo_infusion = ComboBox::new(im_str!("##item_spawn_infu"))
      .preview_value(preview)
      .height(ComboBoxHeight::Large);
    combo_infusion.build(ui, || {
      for (idx, (item, label)) in INFUSION_TYPES.iter().enumerate() {
        let selected = idx == self.infusion_idx;
        if Selectable::new(&label).selected(selected).build(ui) {
          self.infusion_idx = idx;
          self.infusion = *item;
        }
      }
    });

    ui.same_line(0.);
    let preview = &UPGRADES.get(self.upgrade_idx).unwrap_or(&UPGRADES[0]).1;
    let combo_upgrade = ComboBox::new(im_str!("##item_spawn_upgr"))
      .preview_value(preview)
      .height(ComboBoxHeight::Large);
    combo_upgrade.build(ui, || {
      for (idx, (item, label)) in UPGRADES.iter().enumerate() {
        let selected = idx == self.upgrade_idx;
        if Selectable::new(&label).selected(selected).build(ui) {
          self.upgrade_idx = idx;
          self.upgrade = *item;
        }
      }
    });

    w_tok.pop(ui);

    let w_tok = ui.push_item_width(size[0] * 0.925);
    let slider = Slider::new(im_str!("##slider")).range(1u32..=99u32);
    slider.build(ui, &mut self.qty);
    w_tok.pop(ui);

    id_tok.pop(ui);

    false
  }

  fn interact(&mut self, ui: &imgui::Ui, is_active: bool, is_interacting: bool) {
    if self
      .hotkey_spawn
      .map(|k| ui.is_key_released(k as _))
      .unwrap_or(false)
      || is_interacting
    {
      self.spawn()
    }
  }

  fn is_valid(&self) -> bool {
    ItemSpawn::is_valid(self)
  }
}
