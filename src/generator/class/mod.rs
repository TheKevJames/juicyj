use generator::asm::Instr;

pub mod constructor;
pub mod field;
pub mod method;

pub fn code(text: &Vec<String>,
            externs: &Vec<String>,
            bss: &Vec<(String, String)>,
            data: &Vec<String>)
            -> String {
    let mut generated = Vec::new();

    if !externs.is_empty() {
        let mut externs = externs.clone();
        externs.sort();
        externs.dedup();

        // do not import exported labels
        // iterate backward to ensure array deletion doesn't fuck with things
        for (idx, ext) in externs.clone().iter().enumerate().rev() {
            let split = ext.split_whitespace().collect::<Vec<&str>>();
            if split[0] != "extern" {
                continue;
            }

            if externs.contains(&vec!["global", split[1]].join(" ")) {
                externs.remove(idx);
            }
        }

        externs.insert(0, format!("{} .{}", Instr::SECTION, "text"));

        generated.push(externs.join("\n"));
    }

    generated.push(text.join("\n"));

    if !bss.is_empty() {
        let mut bss: Vec<String> = bss.iter()
            .map(|v| format!("{}: resb {}", v.0, "16"))
            .collect();
        bss.sort();
        bss.dedup();

        bss.insert(0, format!("{} .{}", Instr::SECTION, "bss"));

        generated.push(bss.join("\n"));
    }

    // TODO<codegen>: dedup data by value
    if !data.is_empty() {
        let mut data = data.clone();
        data.sort();
        data.dedup();
        data.insert(0, format!("{} .{}", Instr::SECTION, "data"));

        generated.push(data.join("\n"));
    }

    generated.join("\n\n")
}
