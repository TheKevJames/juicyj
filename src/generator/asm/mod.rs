use std::fmt;

pub enum Register {
    // general
    EAX, // Accumulator
    AL,
    EBX, // Base
    BL,
    ECX, // Counter
    CL,
    EDX, // Data
    DL,

    // indexes
    EDI, // Destination
    ESI, // Source

    // pointers
    EBP, // Stack Base
    ESP, // Stack Pointer
    EIP, // Index Pointer
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Register::EAX => write!(f, "{}", "eax"),
            Register::AL => write!(f, "{}", "al"),
            Register::EBX => write!(f, "{}", "ebx"),
            Register::BL => write!(f, "{}", "bl"),
            Register::ECX => write!(f, "{}", "ecx"),
            Register::CL => write!(f, "{}", "cl"),
            Register::EDX => write!(f, "{}", "edx"),
            Register::DL => write!(f, "{}", "dl"),

            Register::EDI => write!(f, "{}", "edi"),
            Register::ESI => write!(f, "{}", "esi"),

            Register::EBP => write!(f, "{}", "ebp"),
            Register::ESP => write!(f, "{}", "esp"),
            Register::EIP => write!(f, "{}", "eip"),
        }
    }
}
