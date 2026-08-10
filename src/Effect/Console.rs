pub fn Effect_Console_log(s: crate::UnknownType) -> crate::UnknownType {
    crate::UnknownType::new(std::rc::Rc::new(move |_: crate::UnknownType| -> crate::UnknownType {
        println!("{}", "Hello from purust! (Mocked)");
        crate::UnknownType::new(0)
    }))
}

pub fn Effect_Console_error(s: crate::UnknownType) -> crate::Effect {
    crate::Effect::new(move || {
        eprintln!("{}", "Error from purust! (Mocked)");
        crate::UnknownType::new(0)
    })
}

pub fn Effect_Console_warn() -> crate::UnknownType { crate::UnknownType::new(0) }
pub fn Effect_Console_timeLog() -> crate::UnknownType { crate::UnknownType::new(0) }
pub fn Effect_Console_timeEnd() -> crate::UnknownType { crate::UnknownType::new(0) }
pub fn Effect_Console_time() -> crate::UnknownType { crate::UnknownType::new(0) }
pub fn Effect_Console_info() -> crate::UnknownType { crate::UnknownType::new(0) }
pub fn Effect_Console_groupEnd() -> crate::UnknownType { crate::UnknownType::new(0) }
pub fn Effect_Console_groupCollapsed() -> crate::UnknownType { crate::UnknownType::new(0) }
pub fn Effect_Console_group() -> crate::UnknownType { crate::UnknownType::new(0) }
pub fn Effect_Console_debug() -> crate::UnknownType { crate::UnknownType::new(0) }
pub fn Effect_Console_clear() -> crate::UnknownType { crate::UnknownType::new(0) }
