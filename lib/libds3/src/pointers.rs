use std::lazy::SyncLazy;

use crate::{memedit::PointerChain, version::VERSION, version::Version};

pub static MOUSE_ENABLE: SyncLazy<PointerChain<u8>> = SyncLazy::new(|| {
    PointerChain::new(&match VERSION.unwrap() {
        Version::Ver104 => [0x1446A9280, 0x54], 
        Version::Ver108 => [0x1447103D8, 0x54],
        Version::Ver112 => [0x144746988, 0x54],
        Version::Ver115 => [0x14474C2E8, 0x54],
    })
});

pub static IGT: SyncLazy<PointerChain<u64>> = SyncLazy::new(|| {
    PointerChain::new(&match VERSION.unwrap() {
        Version::Ver104 => [0x14469D118, 0x9c], 
        Version::Ver108 => [0x144704268, 0xa4],
        Version::Ver112 => [0x14473A818, 0xa4],
        Version::Ver115 => [0x144740148, 0xa4],
    })
});
