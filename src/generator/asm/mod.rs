use std::fmt;

pub mod helper;

pub enum Instr {
    // structure
    EXTERN,
    GLOBAL,
    SECTION,

    // general
    CALL,
    INT,
    MOV,
    POP,
    PUSH,
    RET,

    // math
    ADD,
    DIV,
    MUL,
    SUB,

    // comparison
    AND,
    CMP,
    JE,
    JMP,
    JNE,
    OR,
    XOR,

    // conditional sets
    SETE,
    SETNE,
    SETL,
    SETLE,
    SETG,
    SETGE,
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instr::EXTERN => write!(f, "{}", "extern"),
            Instr::GLOBAL => write!(f, "{}", "global"),
            Instr::SECTION => write!(f, "{}", "section"),

            Instr::CALL => write!(f, "  {}", "call"),
            Instr::INT => write!(f, "  {}", "int"),
            Instr::MOV => write!(f, "  {}", "mov"),
            Instr::POP => write!(f, "  {}", "pop"),
            Instr::PUSH => write!(f, "  {}", "push"),
            Instr::RET => write!(f, "  {}", "ret"),

            Instr::ADD => write!(f, "  {}", "add"),
            Instr::DIV => write!(f, "  {}", "div"),
            Instr::MUL => write!(f, "  {}", "imul"),
            Instr::SUB => write!(f, "  {}", "sub"),

            Instr::AND => write!(f, "  {}", "and"),
            Instr::CMP => write!(f, "  {}", "cmp"),
            Instr::JE => write!(f, "  {}", "je"),
            Instr::JMP => write!(f, "  {}", "jmp"),
            Instr::JNE => write!(f, "  {}", "jne"),
            Instr::OR => write!(f, "  {}", "or"),
            Instr::XOR => write!(f, "  {}", "xor"),

            Instr::SETE => write!(f, "  {}", "sete"),
            Instr::SETNE => write!(f, "  {}", "setne"),
            Instr::SETL => write!(f, "  {}", "setl"),
            Instr::SETLE => write!(f, "  {}", "setle"),
            Instr::SETG => write!(f, "  {}", "setg"),
            Instr::SETGE => write!(f, "  {}", "setge"),
        }
    }
}

pub enum Reg {
    // general
    EAX, // Accumulator
    AL,
    EBX, // Base
    // BL,
    ECX, // Counter
    // CL,
    EDX, // Data
    // DL,

    // indexes
    EDI, // Destination
    ESI, // Source

    // pointers
    EBP, // Stack Base
    ESP, // Stack Pointer
         // EIP, // Index Pointer
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Reg::EAX => write!(f, "{}", "eax"),
            Reg::AL => write!(f, "{}", "al"),
            Reg::EBX => write!(f, "{}", "ebx"),
            // Reg::BL => write!(f, "{}", "bl"),
            Reg::ECX => write!(f, "{}", "ecx"),
            // Reg::CL => write!(f, "{}", "cl"),
            Reg::EDX => write!(f, "{}", "edx"),
            // Reg::DL => write!(f, "{}", "dl"),
            Reg::EDI => write!(f, "{}", "edi"),
            Reg::ESI => write!(f, "{}", "esi"),

            Reg::EBP => write!(f, "{}", "ebp"),
            Reg::ESP => write!(f, "{}", "esp"),
            // Reg::EIP => write!(f, "{}", "eip"),
        }
    }
}
