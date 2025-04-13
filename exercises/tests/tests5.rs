// tests5.rs


/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    let ptr = address as *mut u32;
    // SAFETY: The caller ensures that `address` is a valid pointer to a mutable u32.
    // We dereference the pointer here to modify the value it points to.
    unsafe {
        *ptr = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert_eq!(t, 0xAABBCCDD);
    }
}
