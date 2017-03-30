use generator::asm::Instr;
use generator::asm::Reg;

pub fn go(mut text: &mut Vec<String>) -> Result<Option<String>, String> {
    // TODO<codegen>: actually do this correctly.
    text.push(format!("{} {}, {}", Instr::MOV, Reg::EAX, "0xDEAD"));

    // TODO<codegen>: kind is null
    Ok(None)
}
