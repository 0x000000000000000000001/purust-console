use std::rc::Rc;
use crate::{UnknownType, Record_a};

pub fn Effect_Console_log(mut a0: UnknownType) -> UnknownType {
    perceus_ptr::PerceusPtr::new(Record_a {
        call: Some(Rc::new(move |mut _u: UnknownType| -> UnknownType {
            println!("{}", a0.init_string.as_ref().unwrap());
            UnknownType::new(Record_a { ..Default::default() })
        })),
        ..Default::default()
    })
}

pub fn Effect_Console_error(mut a0: UnknownType) -> UnknownType {
    perceus_ptr::PerceusPtr::new(Record_a {
        call: Some(Rc::new(move |mut _u: UnknownType| -> UnknownType {
            eprintln!("{}", a0.init_string.as_ref().unwrap());
            UnknownType::new(Record_a { ..Default::default() })
        })),
        ..Default::default()
    })
}

pub fn Effect_Console_warn(mut a0: UnknownType) -> UnknownType {
    perceus_ptr::PerceusPtr::new(Record_a {
        call: Some(Rc::new(move |mut _u: UnknownType| -> UnknownType {
            eprintln!("WARN: {}", a0.init_string.as_ref().unwrap());
            UnknownType::new(Record_a { ..Default::default() })
        })),
        ..Default::default()
    })
}

pub fn Effect_Console_info(mut a0: UnknownType) -> UnknownType {
    perceus_ptr::PerceusPtr::new(Record_a {
        call: Some(Rc::new(move |mut _u: UnknownType| -> UnknownType {
            println!("INFO: {}", a0.init_string.as_ref().unwrap());
            UnknownType::new(Record_a { ..Default::default() })
        })),
        ..Default::default()
    })
}

pub fn Effect_Console_debug(mut a0: UnknownType) -> UnknownType {
    perceus_ptr::PerceusPtr::new(Record_a {
        call: Some(Rc::new(move |mut _u: UnknownType| -> UnknownType {
            println!("DEBUG: {}", a0.init_string.as_ref().unwrap());
            UnknownType::new(Record_a { ..Default::default() })
        })),
        ..Default::default()
    })
}

pub fn Effect_Console_clear() -> UnknownType {
    perceus_ptr::PerceusPtr::new(Record_a {
        call: Some(Rc::new(move |mut _u: UnknownType| -> UnknownType {
            UnknownType::new(Record_a { ..Default::default() })
        })),
        ..Default::default()
    })
}

pub fn Effect_Console_group(mut a0: UnknownType) -> UnknownType {
    perceus_ptr::PerceusPtr::new(Record_a {
        call: Some(Rc::new(move |mut _u: UnknownType| -> UnknownType {
            UnknownType::new(Record_a { ..Default::default() })
        })),
        ..Default::default()
    })
}

pub fn Effect_Console_groupCollapsed(mut a0: UnknownType) -> UnknownType {
    perceus_ptr::PerceusPtr::new(Record_a {
        call: Some(Rc::new(move |mut _u: UnknownType| -> UnknownType {
            UnknownType::new(Record_a { ..Default::default() })
        })),
        ..Default::default()
    })
}

pub fn Effect_Console_groupEnd() -> UnknownType {
    perceus_ptr::PerceusPtr::new(Record_a {
        call: Some(Rc::new(move |mut _u: UnknownType| -> UnknownType {
            UnknownType::new(Record_a { ..Default::default() })
        })),
        ..Default::default()
    })
}

pub fn Effect_Console_time(mut a0: UnknownType) -> UnknownType {
    perceus_ptr::PerceusPtr::new(Record_a {
        call: Some(Rc::new(move |mut _u: UnknownType| -> UnknownType {
            UnknownType::new(Record_a { ..Default::default() })
        })),
        ..Default::default()
    })
}

pub fn Effect_Console_timeEnd(mut a0: UnknownType) -> UnknownType {
    perceus_ptr::PerceusPtr::new(Record_a {
        call: Some(Rc::new(move |mut _u: UnknownType| -> UnknownType {
            UnknownType::new(Record_a { ..Default::default() })
        })),
        ..Default::default()
    })
}

pub fn Effect_Console_timeLog(mut a0: UnknownType) -> UnknownType {
    perceus_ptr::PerceusPtr::new(Record_a {
        call: Some(Rc::new(move |mut _u: UnknownType| -> UnknownType {
            UnknownType::new(Record_a { ..Default::default() })
        })),
        ..Default::default()
    })
}
