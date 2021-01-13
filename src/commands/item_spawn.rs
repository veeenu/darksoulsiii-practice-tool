use hudhook::*;

use winapi::um::errhandlingapi;
use winapi::um::memoryapi;
use winapi::um::synchapi;
use winapi::um::processthreadsapi;

use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};
use imgui::ImString;
use log::*;

use super::{Command, BUTTON_HEIGHT, BUTTON_WIDTH};
use crate::config::get_symbol;

pub(crate) struct ItemSpawn {
  label: imgui::ImString,
  hotkey_spawn: Option<i32>,
  hotkey_focus: Option<i32>,
  item_id: u32,
  upgrade: u32,
  qty: u32,
  durability: u32,
  addrs: (u64, u64, u64),
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
    hotkey_spawn: Option<i32>,
    hotkey_focus: Option<i32>,
  ) -> ItemSpawn {
    let upgrade = 0u32;
    let qty = 1u32;
    let durability: u32 = 0xffffffff;

    ItemSpawn {
      label: imgui::ImString::new(label.to_string()),
      hotkey_spawn,
      hotkey_focus,
      item_id: 0x4c4c08,
      upgrade: 6,
      qty: 1,
      durability,
      addrs: (function_ptr, unk_addr1, unk_addr2),
    }
  }

  pub(crate) fn is_valid(&self) -> bool {
    // self.pointer.eval().is_some()
    true
  }

  fn spawn(&self) {
    let mut ops = dynasmrt::x64::Assembler::new().unwrap();
    dynasm!(ops
    ; .arch x64
    ; sub rsp, 0x48
    ; lea r8d, [rsp + 0x20]
    ; lea rdx, [rsp + 0x30]
    ; mov eax, DWORD self.qty as _
    ; mov ebx, DWORD (self.item_id + self.upgrade) as _
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
      debug!("Buffer: {}", (0..105).into_iter().map(|i| format!("{:02x}", bufp[i])).collect::<Vec<_>>().join(" "));
      // for i in 0..105 {
      //   let ptr = buf.ptr(dynasmrt::AssemblyOffset(i));
      //   print!("{:02x} ", unsafe { *ptr });
      // }
      // println!("");
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

      unsafe {
        synchapi::WaitForSingleObject(thread, 0xffffffff)
      };

      let ret = unsafe { memoryapi::VirtualFreeEx(hproc, addr, 0, 0x8000) };

      if ret == 0 {
        error!("VirtualFreeEx: {:x}", unsafe {
          errhandlingapi::GetLastError()
        });
        return;
      }
    });
  }
}

impl Command for ItemSpawn {
  fn display(&self, ui: &imgui::Ui) -> bool {
    ui.button(&self.label, [BUTTON_WIDTH, BUTTON_HEIGHT])
  }

  fn interact(&mut self, ui: &imgui::Ui, is_active: bool, is_interacting: bool) {
    if self
      .hotkey_spawn
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
