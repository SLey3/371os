pub mod lib {
    use std::slice::from_raw_parts_mut;
    use std::cmp::min;

    pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let ptr: *mut i32 = values.as_mut_ptr();
        let mid: usize = min(values.len(), mid);
    
        unsafe {
            (
                from_raw_parts_mut(ptr, mid),
                from_raw_parts_mut(ptr.add(mid), values.len() - mid)
            )
        }
    }
}