use std::slice;

use crate::api::area_object_manager::AreaObjectManagerPtr;

use super::send;

pub unsafe fn handle_demand3(area_object_manager: AreaObjectManagerPtr) {
    let class59 = area_object_manager.get_class59();
    let string_buffer = class59.get_string_buffer();
    let buf = string_buffer.get_buf();
    let string = String::from_utf16_lossy(slice::from_raw_parts(buf as *const u16, string_buffer.get_len() as _));
    send(&format!("handle_demand3 {}\n", string));
    /*
    let string_buffer = class59.get_string_buffer();
    send(&format!("handle_demand3 string_buffer={:#018x}\n", string_buffer.address));
    let buf = string_buffer.get_buf();
    send(&format!("handle_demand3 buf={:#018x}\n", buf));
    */
}
