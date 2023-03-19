/* automatically generated by rust-bindgen 0.64.0 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tm {
    pub tm_sec: ::std::os::raw::c_int,
    pub tm_min: ::std::os::raw::c_int,
    pub tm_hour: ::std::os::raw::c_int,
    pub tm_mday: ::std::os::raw::c_int,
    pub tm_mon: ::std::os::raw::c_int,
    pub tm_year: ::std::os::raw::c_int,
    pub tm_wday: ::std::os::raw::c_int,
    pub tm_yday: ::std::os::raw::c_int,
    pub tm_isdst: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_tm() {
    const UNINIT: ::std::mem::MaybeUninit<tm> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<tm>(),
        36usize,
        concat!("Size of: ", stringify!(tm))
    );
    assert_eq!(
        ::std::mem::align_of::<tm>(),
        4usize,
        concat!("Alignment of ", stringify!(tm))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tm_sec) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_sec)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tm_min) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_min)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tm_hour) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_hour)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tm_mday) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_mday)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tm_mon) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_mon)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tm_year) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_year)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tm_wday) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_wday)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tm_yday) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_yday)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tm_isdst) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_isdst)
        )
    );
}
pub type StructTM = tm;
extern "C" {
    pub fn mktime(arg1: *mut StructTM) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn asctime(arg1: *mut StructTM) -> *mut ::std::os::raw::c_char;
}
