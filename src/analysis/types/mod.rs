use analysis::environment::ClassOrInterfaceEnvironment;
use scanner::ASTNode;
use scanner::ASTNodeImport;
use scanner::Token;
use scanner::TokenKind;

// TODO: ensure steps are unambiguous, error otherwise
pub fn lookup(name: &ASTNode,
              current: &ClassOrInterfaceEnvironment,
              kinds: &Vec<ClassOrInterfaceEnvironment>,
              imports: &Vec<ASTNodeImport>)
              -> Result<ClassOrInterfaceEnvironment, String> {
    let tokens: Vec<Token> = match name.token.kind {
        TokenKind::NonTerminal => {
            let mut tokens = Vec::new();
            for child in &name.children {
                tokens.push(child.token.clone());
            }
            tokens
        }
        _ => vec![name.token.clone()],
    };

    // 1. try the enclosing class or interface
    if let Some((class_name, _)) = current.name.split_last() {
        if &tokens == &vec![class_name.clone()] || &tokens == &current.name {
            return Ok(current.clone());
        }
    }

    // 2. try any single-type-import (A.B.C.D)
    for import in imports {
        if let Some((import_name, _)) = import.import.split_last() {
            if import_name == &Token::new(TokenKind::Star, None) {
                continue;
            }

            if &tokens == &import.import || &tokens == &vec![import_name.clone()] {
                for cls_or_intfc in kinds {
                    if let Some((cls_or_intfc_name, _)) = cls_or_intfc.name.split_last() {
                        if &tokens == &vec![cls_or_intfc_name.clone()] ||
                           &tokens == &cls_or_intfc.name {
                            return Ok(cls_or_intfc.clone());
                        }
                    }
                }

                return Err(format!("could not find kind for imported lookup {:?}",
                                   import.import));
            }
        }
    }

    // 3. try the same package
    for cls_or_intfc in kinds {
        if let Some((cls_or_intfc_name, cls_or_intfc_package)) = cls_or_intfc.name.split_last() {
            if let Some((_, package)) = current.name.split_last() {
                if package == cls_or_intfc_package &&
                   (&tokens == &vec![cls_or_intfc_name.clone()] || &tokens == &cls_or_intfc.name) {
                    return Ok(cls_or_intfc.clone());
                }
            }
        }
    }

    // 4. try any import-on-demand package (A.B.C.*) including java.lang.*
    for import in imports {
        if let Some((import_name, import_package)) = import.import.split_last() {
            if import_name != &Token::new(TokenKind::Star, None) {
                continue;
            }

            for cls_or_intfc in kinds {
                if let Some((cls_or_intfc_name, cls_or_intfc_package)) =
                    cls_or_intfc.name.split_last() {
                    if import_package == cls_or_intfc_package &&
                       (&tokens == &vec![cls_or_intfc_name.clone()] ||
                        &tokens == &cls_or_intfc.name) {
                        return Ok(cls_or_intfc.clone());
                    }
                }
            }
        }
    }

    return Err("could not lookup kind".to_owned());
}
