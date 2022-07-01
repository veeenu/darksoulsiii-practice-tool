use std::sync::LazyLock;

use crate::memedit::PointerChain;
use crate::version::{Version, VERSION};

pub static MOUSE_ENABLE: LazyLock<PointerChain<u8>> = LazyLock::new(|| {
    PointerChain::new(&match VERSION.unwrap() {
        Version::Ver104 => [0x1446A9280, 0x54],
        Version::Ver108 => [0x1447103D8, 0x54],
        Version::Ver112 => [0x144746988, 0x54],
        Version::Ver115 => [0x14474C2E8, 0x54],
    })
});

pub static IGT: LazyLock<PointerChain<u64>> = LazyLock::new(|| {
    PointerChain::new(&match VERSION.unwrap() {
        Version::Ver104 => [0x14469D118, 0x9c],
        Version::Ver108 => [0x144704268, 0xa4],
        Version::Ver112 => [0x14473A818, 0xa4],
        Version::Ver115 => [0x144740148, 0xa4],
    })
});
