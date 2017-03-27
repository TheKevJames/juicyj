use generator::asm::helper::call;
use scanner::ASTNode;

pub fn go(node: &ASTNode,
          class_label: &String,
          label: &String,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    call(&node.children[0],
         &node.children[2],
         class_label,
         label,
         &mut text,
         &mut externs,
         &mut bss,
         &mut data)
}
