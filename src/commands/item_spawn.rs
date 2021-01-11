use hudhook::memory::PointerChain;
use hudhook::*;

use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};
use imgui::ImString;
use log::*;

use super::{Command, BUTTON_HEIGHT, BUTTON_WIDTH};
use crate::config::get_symbol;

pub(crate) struct ItemSpawn {
  label: imgui::ImString,
  hotkey: Option<i32>,
}

impl ItemSpawn {
  pub(crate) fn new(
    label: &str,
    item_id: u32,
    upgrade: u32,
    qty: u32,
    durability: u32,
    function_ptr: u64,
    unk_addr1: u64,
    unk_addr2: u64,
    hotkey: Option<i32>,
  ) -> ItemSpawn {
    let mut ops = dynasmrt::x64::Assembler::new().unwrap();
    let upgrade = 0u32;
    let qty = 1u32;
    let durability: u32 = 0xffffffff;

    let function_ptr = function_ptr;

    dynasm!(ops
      ; .arch x64
      ; sub rsp, 0x48
      ; lea r8d, [rsp + 0x20]
      ; lea rdx, [rsp + 0x30]
      ; mov eax, DWORD qty as _
      ; mov ebx, DWORD (item_id + upgrade) as _
      ; mov esi, DWORD durability as _
      ; mov DWORD [rsp + 0x30], 1u32 as _
      ; mov [rsp + 0x3c], esi
      ; mov [rsp + 0x34], ebx
      ; mov [rsp + 0x38], eax
      ; mov rax, QWORD unk_addr1 as i64
      ; mov rax, [rax]
      ; mov rbx, rax
      ; mov rax, QWORD unk_addr2 as i64
      ; mov rax, [rax]
      ; mov rbp, [rax + 0x80]
      ; mov rcx, rbx
      ; mov rsi, QWORD function_ptr as _
      ; call rsi
      ; add rsp, BYTE 0x48
      ; ret
    );

    let buf = ops.finalize().unwrap();
    info!("Buffer: {:#?}", buf);

    for i in 0..105 {
      let ptr = buf.ptr(dynasmrt::AssemblyOffset(i));
      print!("{:02x} ", unsafe { *ptr });
    }
    println!("");

    ItemSpawn {
      label: imgui::ImString::new(label.to_string()),
      hotkey
    }
  }

  pub(crate) fn is_valid(&self) -> bool {
    // self.pointer.eval().is_some()
    true
  }

  fn spawn(&self) {}
}

impl Command for ItemSpawn {
  fn display(&self, ui: &imgui::Ui) -> bool {
    ui.button(&self.label, [BUTTON_WIDTH, BUTTON_HEIGHT])
  }

  fn interact(&mut self, ui: &imgui::Ui, is_active: bool, is_interacting: bool) {
    if (is_active && is_interacting)
      || self
        .hotkey
        .map(|k| ui.is_key_released(k as _))
        .unwrap_or(false)
    {
      self.spawn()
    }
  }

  fn is_valid(&self) -> bool {
    ItemSpawn::is_valid(self)
  }
}
