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
use std::fs;
use std::process::exit;

use crate::utils::{get_bytes_to_u16, get_bytes_to_u32};
use crate::constants::{JDK_LANGUAGE_NAME, JDK_MAJOR_BASIC_VERSION, JAVA_MAGIC_NUMBER};

pub struct JavaClass {
    version: JavaVersion,

}

impl JavaClass {
    pub fn new(class_bytes: &mut Vec<ClassByte>) -> Self {
        let minor_version = get_bytes_to_u16(class_bytes);
        let major_version = get_bytes_to_u16(class_bytes);
        let java_version = JavaVersion::new(minor_version, major_version);

        println!("java minor version: {}", java_version.minor_version);
        println!("java language version: {}", java_version.major_language_version());

        JavaClass {
            version: java_version
        }
    }
}


impl JavaVersion {
    fn new(minor_version: u16, major_version: u16) -> Self {
        JavaVersion {
            minor_version,
            major_version,
        }
    }

    fn major_language_version(&self) -> String {
        let java_language_version = self.major_version - JDK_MAJOR_BASIC_VERSION + 1;
        let mut java_language_version_desc = String::new();
        java_language_version_desc.push_str(JDK_LANGUAGE_NAME);
        java_language_version_desc.push_str(" ");
        java_language_version_desc.push_str(&java_language_version.to_string());
        java_language_version_desc
    }
}

pub struct JavaVersion {
    minor_version: u16,
    major_version: u16,
}

type ClassByte = u8;

// # load the java class file with the target file system path
pub fn parse(class_location: &str) -> JavaClass {
    let mut class_bytes: Vec<ClassByte> = fs::read(class_location)
        .unwrap_or_else(
            |err| {
                println!("can not get the class file in: {}, reason is: {}, just ended.", class_location, err);
                exit(-1);
            });
    let result = check_magic_number(&mut class_bytes);
    println!("check result: {}", result);
    JavaClass::new(&mut class_bytes)
}

fn check_magic_number(class_bytes: &mut Vec<ClassByte>) -> bool {
    //get the length, if the length shorter than the magic number that means
    //the target class bytes is not a valid class of course
    if class_bytes.len() < 4 {
        return false;
    }
    //CAFE BABE
    let magic_number = get_bytes_to_u32(class_bytes);
    magic_number == JAVA_MAGIC_NUMBER
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