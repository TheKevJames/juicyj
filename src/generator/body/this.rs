use generator::asm::Instr;
use generator::asm::Reg;

pub fn go(class_label: &String,
          mut text: &mut Vec<String>,
          mut bss: &mut Vec<String>)
          -> Result<(), String> {
    let this = format!("{}.THIS", class_label);
    bss.push(this.clone());

    text.push(format!("  ; this"));
    text.push(format!("{} {}, [{}]", Instr::MOV, Reg::ESI, &this));
    text.push(format!("{} {}, [{}]", Instr::MOV, Reg::EAX, Reg::ESI));
    text.push("".to_owned());

    Ok(())
}
