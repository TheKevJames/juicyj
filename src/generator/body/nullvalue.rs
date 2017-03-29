use generator::asm::Instr;
use generator::asm::Reg;

pub fn go(mut text: &mut Vec<String>) -> Result<(), String> {
    // TODO<codegen>: actually do this correctly.
    text.push(format!("{} {}, {}", Instr::MOV, Reg::EAX, "-42"));

    Ok(())
}
