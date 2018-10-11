use std::alloc::{alloc, Layout};
use std::mem::{size_of, transmute};
use super::native::*;

struct Ftdi2xx {}

impl Ftdi2xx {

    fn allocate_list_info_node(num_devices : usize) {
        let struct_size = size_of::<_ft_device_list_info_node>();
        let alloc_size = struct_size * num_devices;
        let layout = Layout::from_size_align(alloc_size, 8).expect("Unable to allocate");
        let data : Vec<_ft_device_list_info_node> = unsafe {
            let raw_data = alloc(layout);
            Vec::from_raw_parts(
                transmute::<[u8], [_ft_device_list_info_node]>(raw_data),
                alloc_size, alloc_size)
        };
        // println!("Allocated {:?}\n{:?}", alloc_size, data);
    }

    fn get_device_list() {
        unsafe {
            let mut num_devs: u32 = 0;
            let mut status: FT_STATUS = FT_CreateDeviceInfoList(&mut num_devs);
            println!("Found {:?} device(s)", num_devs);
            Ftdi2xx::allocate_list_info_node(num_devs as usize);

            // assert!(status == FT_OK);
            // status = FT_GetDeviceInfoList(&mut info_node, &mut num_devs);
            // println!("{:?} = {:?}", status, num_devs);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_device_list() {
        Ftdi2xx::get_device_list();
    }
}
