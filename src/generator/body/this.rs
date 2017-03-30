use generator::asm::Instr;
use generator::asm::Reg;

pub fn go(class_label: &String,
          mut text: &mut Vec<String>,
          mut bss: &mut Vec<String>)
          -> Result<(), String> {
    text.push(format!("  ; this"));
    text.push(format!("{} {}, {}", Instr::MOV, Reg::ESI, Reg::EBX));
    text.push(format!("{} {}, {}", Instr::MOV, Reg::EAX, Reg::EBX));
    text.push("".to_owned());

    Ok(())
}
