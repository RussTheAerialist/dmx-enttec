use std::result;
use std::fmt;
use std::error::Error;
use std::alloc::{alloc, Layout};
use std::mem::{size_of};
use super::native::*;

#[derive(Debug)]
pub enum FtdiError {
    Generic,
    UnableToAllocate,
    NoDevicesFound,
}
impl Error for FtdiError {
    fn description(&self) -> &str {
        match self {
            FtdiError::Generic => "Generic FTDI Error",
            FtdiError::UnableToAllocate => "Unable to allocated needed memory",
            FtdiError::NoDevicesFound => "No FDTI devices detected",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
impl fmt::Display for FtdiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

type Result<T> = result::Result<T, FtdiError>;

pub struct Ftdi2xx {}

impl Ftdi2xx {

    pub fn allocate_list_info_node(num_devices : usize) {
        type Node = _ft_device_list_info_node;

        let struct_size = size_of::<Node>();
        let alloc_size = struct_size * num_devices;
        let layout = Layout::from_size_align(alloc_size, 8).expect("Unable to allocate");

        #[allow(clippy::cast_ptr_alignment)]
        let data : Vec<Node> = unsafe {
            let raw_data = (alloc(layout) as *mut u8) as *mut Node;
            Vec::<Node>::from_raw_parts(raw_data, num_devices, num_devices)
        };
        println!("Allocated {:?}\n{:?}", alloc_size, data);
    }

    pub fn get_device_list() -> Result<Vec<_ft_device_list_info_node>> {
        // TODO: Reduce the footprint of this unsafe block
        unsafe {
            let mut num_devs: u32 = 0;
            let _status: FT_STATUS = FT_CreateDeviceInfoList(&mut num_devs);
            // TODO: Use log::debug instead
            println!("Found {:?} device(s)", num_devs);
            if num_devs == 0 {
                return Err(FtdiError::NoDevicesFound)
            }
            Ftdi2xx::allocate_list_info_node(num_devs as usize);

            // assert!(status == FT_OK);
            // status = FT_GetDeviceInfoList(&mut info_node, &mut num_devs);
            // println!("{:?} = {:?}", status, num_devs);
        }
        Err(FtdiError::Generic)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_device_list() {
        Ftdi2xx::get_device_list().unwrap();
    }
}
