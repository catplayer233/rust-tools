//!
//! ```
//! ClassFile {
//!     u4             magic;
//!     u2             minor_version;
//!     u2             major_version;
//!     u2             constant_pool_count;
//!     cp_info        constant_pool[constant_pool_count-1];
//!     u2             access_flags;
//!     u2             this_class;
//!     u2             super_class;
//!     u2             interfaces_count;
//!     u2             interfaces[interfaces_count];
//!     u2             fields_count;
//!     field_info     fields[fields_count];
//!     u2             methods_count;
//!     method_info    methods[methods_count];
//!     u2             attributes_count;
//!     attribute_info attributes[attributes_count];
//! }
//! ```
use std::convert::TryInto;
use std::fs;
use std::process::exit;

use crate::constant_pool::parser::{ConstantPool, parse_constant_pool};
use crate::constants::{ClassByte, JAVA_MAGIC_NUMBER, JDK_LANGUAGE_NAME, JDK_MAJOR_BASIC_VERSION};
use crate::utils::{get_bytes_to_u16, get_bytes_to_u32};
use crate::version::parser::{JavaVersion, parse_java_version};

pub struct JavaClass {
    version: JavaVersion,
    constant_pool: ConstantPool,
}

//api for class
impl JavaClass {
    pub fn new(class_bytes: &mut Vec<ClassByte>) -> Self {
        let version = parse_java_version(class_bytes);
        let constant_pool = parse_constant_pool(class_bytes);

        JavaClass {
            version,
            constant_pool,
        }
    }
}


// # load the java class file with the target file system path
pub fn parse(class_location: &str) -> JavaClass {
    let mut class_bytes: Vec<ClassByte> = fs::read(class_location)
        .unwrap_or_else(
            |err| {
                println!("can not get the class file in: {}, reason is: {}, just ended.", class_location, err);
                exit(-1);
            });
    check_magic_number(&mut class_bytes).map_err(|error_message|
        println!("the class file is invalid: {}", error_message));
    JavaClass::new(&mut class_bytes)
}

fn check_magic_number(class_bytes: &mut Vec<ClassByte>) -> Result<(), String> {
    //get the length, if the length shorter than the magic number that means
    //the target class bytes is not a valid class of course
    if class_bytes.len() < 4 {
        return Err(String::from("the class bytes' size is smaller than magic_number's length: 4"));
    }
    //CAFE BABE
    let magic_number = get_bytes_to_u32(class_bytes);
    if magic_number == JAVA_MAGIC_NUMBER {
        Ok(())
    } else {
        Err(String::from("the magic number is not as same as 0xCAFEBABE"))
    }
}

#[cfg(test)]
mod test {
    use crate::parser::parse;

    #[test]
    fn class_format_verify() {
        let class_location = "Asserts.class";
        parse(class_location);
    }
}