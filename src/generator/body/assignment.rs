use scanner::ASTNode;

use super::statement;

pub fn go(node: &ASTNode,
          label: &String,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    let variable = match node.children[0].to_label() {
        Ok(l) => format!("{}{}", label, l),
        Err(e) => return Err(e),
    };

    if bss.contains(&variable) {
        text.push(format!("  ; {} = x", variable));

        // get rhs
        match statement::go(&node.children[2],
                            label,
                            &mut text,
                            &mut externs,
                            &mut bss,
                            &mut data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        text.push(format!("  mov {}, [{}]", "edi", variable));
        text.push(format!("  mov [{}], {}", "edi", "eax"));
        text.push("".to_owned());

        return Ok(());
    }

    // TODO: non-local lookup
    Ok(())
}
