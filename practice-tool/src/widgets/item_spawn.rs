use std::borrow::Cow;
use std::fmt::Display;
use std::lazy::SyncLazy;

use dynasmrt::{dynasm, DynasmApi};
use imgui::{ChildWindow, Condition, ListBox, PopupModal, Selectable, Slider, WindowFlags};
use log::{debug, error};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer};
use winapi::um::{errhandlingapi, memoryapi, processthreadsapi, synchapi};

use crate::memedit::Bitflag;
use crate::util::KeyState;

use super::Widget;

const ISP_TAG: &'static str = "##item-spawn";
static ITEM_ID_TREE: SyncLazy<ItemIDTree> =
    SyncLazy::new(|| serde_json::from_str(include_str!("item_ids.json")).unwrap());

static INFUSION_TYPES: [(u32, &'static str); 16] = [
    (0, "Normal"),
    (100, "Heavy"),
    (200, "Sharp"),
    (300, "Refined"),
    (400, "Simple"),
    (500, "Crystal"),
    (600, "Fire"),
    (700, "Chaos"),
    (800, "Lightning"),
    (900, "Deep"),
    (1000, "Dark"),
    (1100, "Poison"),
    (1200, "Blood"),
    (1300, "Raw"),
    (1400, "Blessed"),
    (1500, "Hollow"),
];

static UPGRADES: [(u32, &'static str); 11] = [
    (0, "+0"),
    (1, "+1"),
    (2, "+2"),
    (3, "+3"),
    (4, "+4"),
    (5, "+5"),
    (6, "+6"),
    (7, "+7"),
    (8, "+8"),
    (9, "+9"),
    (10, "+10"),
];

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ItemIDTree {
    Node {
        node: String,
        children: Vec<ItemIDTree>,
    },
    Leaf {
        id: HexU32,
        desc: String,
    },
}

#[derive(Debug)]
struct Stack<'a> {
    stack: Vec<(&'a ItemIDTree, usize)>,
}

impl<'a> Stack<'a> {
    fn new(tree: &'a ItemIDTree) -> Self {
        Stack {
            stack: vec![(tree, 0)],
        }
    }

    fn enter(&mut self, idx: usize) {
        let (last_tree, last_idx) = self.stack.last_mut().unwrap();
        match last_tree {
            ItemIDTree::Node { children, .. } => {
                if idx < children.len() {
                    *last_idx = idx;
                    if let ItemIDTree::Node { .. } = children[idx] {
                        self.stack.push((&children[idx], 0));
                    }
                }
            }
            ItemIDTree::Leaf { .. } => unreachable!(),
        }
    }

    fn exit(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
        }
    }

    fn current(&self) -> Option<&ItemIDTree> {
        match self.stack.last()? {
            &(ItemIDTree::Node { children, .. }, idx) => {
                if idx >= children.len() {
                    None
                } else {
                    Some(&children[idx])
                }
            }
            _ => unreachable!(),
        }
    }

    fn values(&self) -> impl IntoIterator<Item = (usize, bool, &ItemIDTree)> {
        let (last_tree, last_idx) = self.stack.last().unwrap();
        match last_tree {
            ItemIDTree::Node { children, .. } => children
                .iter()
                .enumerate()
                .map(|(idx, node)| (idx, idx == *last_idx, node)),
            ItemIDTree::Leaf { .. } => unreachable!(), // no Leaf is ever pushed on the stack
        }
    }

    fn breadcrumbs(&self) -> String {
        if self.stack.len() == 1 {
            String::from(" / ")
        } else {
            let mut breadcrumbs = String::new();
            for e in self.stack[..self.stack.len()].iter().skip(1) {
                breadcrumbs.extend(" / ".chars());
                breadcrumbs.extend(match e {
                    (ItemIDTree::Node { node, .. }, _) => node.chars(),
                    _ => unreachable!(),
                });
            }
            breadcrumbs
        }
    }
}

#[derive(Debug)]
pub(crate) struct ItemSpawner<'a> {
    label: String,
    key_back: KeyState,
    key_close: KeyState,
    key_load: KeyState,
    stack: Stack<'a>,
    breadcrumbs: String,
    spawn_instance: ItemSpawnInstance,
    sentinel: Bitflag<u8>,
    infusion_idx: usize,
    upgrade_idx: usize,
    log: Option<Vec<String>>,
}

impl<'a> ItemSpawner<'a> {
    pub(crate) fn new(
        addrs: (u64, u64, u64),
        sentinel: Bitflag<u8>,
        key_load: KeyState,
        key_back: KeyState,
        key_close: KeyState,
    ) -> Self {
        let label = format!("Item Spawn (spawn with {})", key_load);
        let stack = Stack::new(&ITEM_ID_TREE);

        let spawn_instance = ItemSpawnInstance {
            addrs,
            qty: 1,
            durability: 100,
            item_id: 0,
            infusion: 0,
            upgrade: 0,
        };

        ItemSpawner {
            label,
            key_back,
            key_close,
            key_load,
            stack,
            log: None,
            breadcrumbs: " /".to_string(),
            spawn_instance,
            sentinel,
            infusion_idx: 0,
            upgrade_idx: 0,
        }
    }

    fn spawn_item(&mut self) {
        let id = if let Some(ItemIDTree::Leaf { id, desc }) = self.stack.current() {
            Some((id.0, desc.clone()))
        } else {
            None
        };

        if self.sentinel.get().is_none() {
            self.write_log("Not spawning item when not in game".into());
            return;
        }

        if let Some((id, desc)) = id {
            self.spawn_instance.item_id = id;
            self.spawn_instance.infusion = INFUSION_TYPES[self.infusion_idx].0;
            self.spawn_instance.upgrade = UPGRADES[self.upgrade_idx].0;
            self.write_log(format!("Spawning {}: {}", desc, self.spawn_instance));
            self.spawn_instance.spawn();
        }
    }

    fn write_log(&mut self, log: String) {
        let logs = self.log.take();
        self.log = match logs {
            Some(mut v) => {
                v.push(log);
                Some(v)
            }
            None => Some(vec![log]),
        };
    }
}

impl<'a> Widget for ItemSpawner<'a> {
    fn render(&mut self, ui: &imgui::Ui) {
        if ui.button_with_size(&self.label, [super::BUTTON_WIDTH, super::BUTTON_HEIGHT]) {
            ui.open_popup(ISP_TAG);
        }
        let [cx, cy] = ui.cursor_pos();
        let [wx, wy] = ui.window_pos();
        let [x, y] = [cx + wx, cy + wy - super::BUTTON_HEIGHT];
        unsafe {
            imgui_sys::igSetNextWindowPos(
                imgui_sys::ImVec2 { x, y },
                Condition::Always as _,
                imgui_sys::ImVec2 { x: 0., y: 0. },
            )
        };

        let style_tokens =
            [ui.push_style_color(imgui::StyleColor::ModalWindowDimBg, [0., 0., 0., 0.])];

        if let Some(_token) = PopupModal::new(ISP_TAG)
            .flags(
                WindowFlags::NO_TITLE_BAR
                    | WindowFlags::NO_RESIZE
                    | WindowFlags::NO_MOVE
                    | WindowFlags::NO_SCROLLBAR
                    | WindowFlags::ALWAYS_AUTO_RESIZE,
            )
            .begin_popup(ui)
        {
            ChildWindow::new("##item-spawn-breadcrumbs")
                .size([240., 14.])
                .build(ui, || {
                    ui.text(&self.breadcrumbs);
                    ui.set_scroll_x(ui.scroll_max_x());
                });

            ListBox::new("##item-spawn-list")
                .size([240., 100.])
                .build(ui, || {
                    if Selectable::new(format!(".. Up one dir ({})", self.key_back)).build(ui) {
                        self.stack.exit();
                    }

                    let mut goto: Option<usize> = None;
                    for (idx, is_selected, i) in self.stack.values() {
                        let repr = match i {
                            ItemIDTree::Node { node, .. } => node,
                            ItemIDTree::Leaf { desc, .. } => desc,
                        };
                        if Selectable::new(repr).selected(is_selected).build(ui) {
                            goto = Some(idx);
                        }
                    }

                    if let Some(idx) = goto {
                        self.stack.enter(idx);
                        self.breadcrumbs = self.stack.breadcrumbs();
                    }
                });

            Slider::new("Qty", 0, 256).build(ui, &mut self.spawn_instance.qty);
            Slider::new("Dur", 0, 9999).build(ui, &mut self.spawn_instance.durability);

            ui.set_next_item_width(240.);

            ui.combo(
                "##item-spawn-infusion",
                &mut self.infusion_idx,
                &INFUSION_TYPES,
                |(_, label)| Cow::Borrowed(label),
            );

            ui.set_next_item_width(240.);

            ui.combo(
                "##item-spawn-upgrade",
                &mut self.upgrade_idx,
                &UPGRADES,
                |(_, label)| Cow::Borrowed(label),
            );

            if ui.button_with_size(format!("Spawn item ({})", self.key_load), [240., 20.]) {
                self.spawn_item();
            }

            if self.key_close.keyup()
                || ui.button_with_size(format!("Close ({})", self.key_close), [240., 20.])
            {
                ui.close_current_popup();
            }
        }

        style_tokens.into_iter().rev().for_each(|t| t.pop());
    }

    fn interact(&mut self) {
        if self.key_back.keyup() {
            self.stack.exit();
            self.breadcrumbs = self.stack.breadcrumbs();
        } else if self.key_load.keyup() {
            self.spawn_item();
        }
    }

    fn log(&mut self) -> Option<Vec<String>> {
        self.log.take()
    }
}

#[derive(Debug)]
struct HexU32(u32);

impl Display for HexU32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

impl<'de> Deserialize<'de> for HexU32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HexVisitor;

        impl<'de> Visitor<'de> for HexVisitor {
            type Value = HexU32;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a 4-byte (8 chars) hexadecimal string")
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.len() != 8 {
                    return Err(E::custom(format!(
                        "Invalid hex string length {}: {}",
                        v.len(),
                        v
                    )));
                }

                let mut bytes = [0u8; 4];
                hex::decode_to_slice(v, &mut bytes[..])
                    .map_err(|e| E::custom(format!("Hex decode error for {}: {}", v, e)))?;
                Ok(HexU32(u32::from_be_bytes(bytes)))
            }
        }

        deserializer.deserialize_any(HexVisitor)
    }
}

#[derive(Debug)]
struct ItemSpawnInstance {
    addrs: (u64, u64, u64),
    qty: u32,
    durability: u32,
    item_id: u32,
    infusion: u32,
    upgrade: u32,
}

impl Display for ItemSpawnInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:08x} (qty={}, dur={}, infusion={}, upgrade={})",
            self.item_id, self.qty, self.durability, self.infusion, self.upgrade
        )
    }
}

impl ItemSpawnInstance {
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
}
