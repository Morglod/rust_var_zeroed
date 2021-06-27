// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

#[macro_export]
macro_rules! var_stack_zeroed {
    ($var_name: ident, $var_type: ty) => {
        let mut $var_name: &mut $var_type;
        unsafe {
            $var_name = &mut *(((&mut ([ 0 as u8; std::mem::size_of::<$var_type>() ])) as *mut [u8; std::mem::size_of::<$var_type>()]) as *mut $var_type)
        }
    };
}

#[macro_export]
macro_rules! var_heap_zeroed {
    ($var_name: ident, $var_type: ty) => {
        let mut $var_name: *mut $var_type;
        unsafe {
            let layout = std::alloc::Layout::new::<$var_type>();
            $var_name = std::alloc::alloc_zeroed(layout) as *mut $var_type;
        }
    };
}
