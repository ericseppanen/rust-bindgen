#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct contains_bool {
    pub a_byte: u8,
    pub a_bool: u8,
}
#[test]
fn bindgen_test_layout_contains_bool() {
    assert_eq!(
        ::std::mem::size_of::<contains_bool>(),
        2usize,
        concat!("Size of: ", stringify!(contains_bool))
    );
    assert_eq!(
        ::std::mem::align_of::<contains_bool>(),
        1usize,
        concat!("Alignment of ", stringify!(contains_bool))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<contains_bool>())).a_byte as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(contains_bool),
            "::",
            stringify!(a_byte)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<contains_bool>())).a_bool as *const _
                as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(contains_bool),
            "::",
            stringify!(a_bool)
        )
    );
}
