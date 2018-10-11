#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::fmt;
use std::ffi::CStr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl fmt::Debug for _ft_device_list_info_node {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Device({} {:?})", self.ID, self.SerialNumber);
        Ok(())
    }
}