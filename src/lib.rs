extern crate libc;
extern crate failure;

#[cfg(target_os = "macos")]
extern crate mach;
#[cfg(target_os = "macos")]
extern crate libproc;
#[cfg(windows)]
extern crate winapi;

#[cfg(target_os = "macos")]
pub mod mac_maps;
#[cfg(target_os = "macos")]
pub use mac_maps::{get_process_maps, MapRange, Pid};

#[cfg(target_os = "linux")]
pub mod linux_maps;
#[cfg(target_os = "linux")]
pub use linux_maps::{get_process_maps, MapRange, Pid};

#[cfg(windows)]
pub mod win_maps;
#[cfg(windows)]
pub use win_maps::{get_process_maps, MapRange, Pid};


fn map_contain_addr(map: &MapRange, addr: usize) -> bool {
    let start = map.start();
    (addr > start) && (addr < (start + map.size()))
}

pub fn maps_contain_addr(addr: usize, maps: &[MapRange]) -> bool {
    maps.iter().any({ |map| map_contain_addr(map, addr) })
}
