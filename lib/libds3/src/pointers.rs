use std::lazy::SyncLazy;

use crate::{memedit::PointerChain, version::VERSION, version::Version};

pub static MOUSE_ENABLE: SyncLazy<PointerChain<u8>> = SyncLazy::new(|| {
    let chain = match VERSION.unwrap() {
        Version::Ver104 => [0x1446A9280, 0x54], 
        Version::Ver108 => [0x1447103D8, 0x54],
        Version::Ver112 => [0x144746988, 0x54],
        Version::Ver115 => [0x14474C2E8, 0x54],
    };
    PointerChain::new(&chain)
});
