pub mod constructor;
pub mod method;

pub fn code(text: &Vec<String>,
            externs: &Vec<String>,
            bss: &Vec<String>,
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

        externs.insert(0, format!("section .text"));

        generated.push(externs.join("\n"));
    }

    generated.push(text.join("\n"));

    if !bss.is_empty() {
        let mut bss = bss.clone();
        bss.sort();
        bss.dedup();

        bss = bss.iter().map(|v| format!("  {}: resb {}", v, "32")).collect();
        bss.insert(0, format!("section .bss"));

        generated.push(bss.join("\n"));
    }

    if !data.is_empty() {
        let mut data = data.clone();
        data.sort();
        data.dedup();
        data.insert(0, format!("section .data"));

        generated.push(data.join("\n"));
    }

    generated.join("\n\n")
}
