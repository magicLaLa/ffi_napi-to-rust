use safer_ffi::prelude::*;

/* Export a Rust function to the C world. */
/// Returns f64;
#[ffi_export]
pub fn get_f64(n: f64, m: f64) -> f64 {
    n * m
}

/// Returns String;
#[ffi_export]
pub fn get_string(n: f64, m: f64) -> char_p::Box {
    format!("string is {}", n * m).try_into().unwrap()
}

/// free String;
#[ffi_export]
pub fn drop_string(string: char_p::Box) {
    drop(string)
}

/* Export a Rust enum to C */
#[derive_ReprC]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum CanvasStartPostion {
    LeftTop = 1,
    LeftBottom = 2,
    RightTop = 3,
    RightBottom = 4,
}

/// A `struct` usable from both Rust and C
#[derive_ReprC]
#[repr(C)]
#[derive(Debug)]
pub struct TestConfig {
    pub gain: f64,
    pub speed: f64,
    pub cell_mg: f64,
    pub canvas_width: f64,
    pub canvas_height: f64,
    pub canvas_position: CanvasStartPostion,
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug)]
pub struct TestInfo {
    pub config: TestConfig,
    pub test_struct: c_slice::Raw<TestStruct>,
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug)]
pub struct TestStruct {
    pub gain: f64,
    pub speed: f64,
    pub name: char_p::Raw,
    pub is_show: bool,
    pub list: c_slice::Raw<f64>,
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug)]
pub struct ReturnStruct {
    pub num: f64,
    pub desc: char_p::Box,
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct ReturnStructv2 {
    pub num: f64,
    pub desc: char_p::Box,
    pub is_show: bool,
}

/// handle `struct` parameters
#[ffi_export]
pub fn process_test_info(test_info: TestInfo) {
    println!("test_info(rust): {test_info:?}");
    let list = unsafe { test_info.test_struct.as_ref() }.as_slice();
    println!("test_info_list(rust): {list:?}");
    let name = unsafe { list[0].name.as_ref().to_str() };
    println!("name(rust): {name:?}");
}

/// Returns the Struct -> ReturnStructv2
#[ffi_export]
pub fn get_test_struct() -> ReturnStructv2 {
    ReturnStructv2 {
        num: 0.2,
        desc: char_p::new("test_name"),
        is_show: false,
    }
}

/// handle array parameter
#[ffi_export]
pub fn test_array(array: c_slice::Raw<TestStruct>) -> repr_c::Vec<ReturnStruct> {
    let list = unsafe { array.as_ref() }.as_slice();
    println!("p(rust): {list:?}");
    let name = unsafe { list[0].name.as_ref().to_str() };
    println!("name(rust): {name:?}");

    let flatten_list = list
        .iter()
        .flat_map(|item| unsafe { item.list.as_ref().as_slice() })
        .collect::<Vec<&f64>>();

    println!("flatten_list(rust): {flatten_list:?}");

    // test i64 as usize to get vec
    let idx: i64 = 1;
    println!("flatten_list_to_get(rust): {:?}", flatten_list.get(idx as usize).unwrap());

    let mut tmp_list = Vec::new();
    for (idx, ele) in list.iter().enumerate() {
        tmp_list.push(ReturnStruct {
            num: ele.speed * ele.gain,
            desc: char_p::new(format!("{idx}. test desc")),
        });
    }
    // https://stackoverflow.com/questions/72325860/cant-pass-array-from-rust-to-node-ffi
    println!("tmp(rust) is {tmp_list:?}");
    tmp_list.into()
}

#[ffi_export]
pub fn rust_free_return_struct(array: repr_c::Vec<ReturnStruct>) {
    drop(array)
}

#[cfg(feature = "generate-headers")]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("headers.h")? // Feel free to rename to your liking (you could even read from an env var)
        .generate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_is_success() {
        let result = get_f64(2.0, 4.0);
        assert_eq!(result, 8.0);
        let str = get_string(2.0, 4.0);
        assert_eq!(str, char_p::new("string is 8"));
        let test_struct = get_test_struct();
        assert_eq!(
            test_struct,
            ReturnStructv2 {
                num: 0.2,
                desc: char_p::new("test_name"),
                is_show: false
            }
        );
    }
}
