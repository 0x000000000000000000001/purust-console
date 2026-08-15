pub fn Effect_Console_log(s: crate::UnknownType) -> crate::UnknownType {
    println!("{}", s.init_string.as_ref().unwrap());
    crate::UnknownType::new(crate::Record_a { ..Default::default() })
}

pub fn Effect_Console_warn(s: crate::UnknownType) -> crate::UnknownType {
    eprintln!("WARN: {}", s.init_string.as_ref().unwrap());
    crate::UnknownType::new(crate::Record_a { ..Default::default() })
}

pub fn Effect_Console_error(s: crate::UnknownType) -> crate::UnknownType {
    eprintln!("ERROR: {}", s.init_string.as_ref().unwrap());
    crate::UnknownType::new(crate::Record_a { ..Default::default() })
}

pub fn Effect_Console_info(s: crate::UnknownType) -> crate::UnknownType {
    println!("INFO: {}", s.init_string.as_ref().unwrap());
    crate::UnknownType::new(crate::Record_a { ..Default::default() })
}
