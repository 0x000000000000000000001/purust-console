pub fn Effect_Console_log(s: String) -> crate::UnknownType {
    println!("{}", s);
    crate::UnknownType::new(Record_a { ..Default::default() })
}

pub fn Effect_Console_warn(s: String) -> crate::UnknownType {
    eprintln!("WARN: {}", s);
    crate::UnknownType::new(Record_a { ..Default::default() })
}

pub fn Effect_Console_error(s: String) -> crate::UnknownType {
    eprintln!("ERROR: {}", s);
    crate::UnknownType::new(Record_a { ..Default::default() })
}

pub fn Effect_Console_info(s: String) -> crate::UnknownType {
    println!("INFO: {}", s);
    crate::UnknownType::new(Record_a { ..Default::default() })
}
