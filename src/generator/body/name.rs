use scanner::ASTNode;

pub fn go(node: &ASTNode,
          label: &String,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    let variable = match node.to_label() {
        Ok(l) => format!("{}{}", label, l),
        Err(e) => return Err(e),
    };

    if bss.contains(&variable) {
        text.push(format!("  ; {}", variable));

        text.push(format!("  mov {}, [{}]", "esi", variable));
        text.push(format!("  mov {}, [{}]", "eax", "esi"));
        text.push("".to_owned());

        return Ok(());
    }

    // TODO: non-local lookup
    Ok(())
}
