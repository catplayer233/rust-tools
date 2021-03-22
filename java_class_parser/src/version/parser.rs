use std::fmt::{Display, Formatter, Result};

use crate::constants::{ClassByte, JDK_LANGUAGE_NAME, JDK_MAJOR_BASIC_VERSION};
use crate::utils::get_bytes_to_u16;

pub struct JavaVersion {
    minor_version: u16,
    major_version: u16,
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

impl Display for JavaVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let major_display = self.major_language_version();
        f.write_str("java version:\n")?;
        f.write_str("major: ")?;
        f.write_str(&major_display)?;
        f.write_str("\n")?;
        f.write_str("minor: ")?;
        f.write_str(&self.minor_version.to_string())
    }
}

pub fn parse_java_version(class_bytes: &mut Vec<ClassByte>) -> JavaVersion {
    let minor_version = get_bytes_to_u16(class_bytes);
    let major_version = get_bytes_to_u16(class_bytes);
    let version = JavaVersion::new(minor_version, major_version);
    println!("{}", version);
    version
}