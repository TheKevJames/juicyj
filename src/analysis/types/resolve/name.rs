use analysis::environment::ClassOrInterface;
use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::lookup;
use analysis::types::obj::Type;
use scanner::ASTNode;
use scanner::TokenKind;

// Sometimes "Name" can be a field access. TODO: fix this in grammar
// Cases: x, this.x, other.x, x.field, this.x.field, other.x.field
pub fn go(node: &ASTNode,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>,
          globals: &Vec<VariableEnvironment>)
          -> Result<Type, String> {
    let mut node = node.clone();
    node.flatten();

    let mut node_fieldless = node.clone();
    let mut field = None;
    if node_fieldless.children.len() >= 3 {
        field = node_fieldless.children.pop();
        node_fieldless.children.pop();
    }

    for var in globals {
        // this.x
        if var.name == node {
            return Ok(Type::new(ClassOrInterfaceEnvironment::new(var.kind.clone(),
                                                                 ClassOrInterface::CLASS)));
        }

        // x
        if var.name.children.len() == 3 &&
           var.name.children[0].clone().token.kind == TokenKind::This &&
           var.name.children[2] == node {
            return Ok(Type::new(ClassOrInterfaceEnvironment::new(var.kind.clone(),
                                                                 ClassOrInterface::CLASS)));
        }

        // other.x
        if node.children[0].clone().token.kind != TokenKind::This && var.name == node.children[0] {
            // at this point, var.name is other.
            // lookup var.kind for a class, then find node.children[2]
            let mut kind = var.kind.clone();
            kind.flatten();
            return match lookup::class::in_env(&kind, current, kinds) {
                Ok(f) => {
                    let mut result = None;

                    for field in &f.fields {
                        if field.name != node.children[2] {
                            continue;
                        }

                        result = Some(field.to_variable().kind.clone());
                    }

                    match result {
                        Some(k) => {
                            let kind = ClassOrInterfaceEnvironment::new(k, ClassOrInterface::CLASS);
                            Ok(Type::new(kind))
                        }
                        None => Err(format!("could not find(1) field {} on {}", node, f)),
                    }
                }
                Err(e) => Err(e),
            };
        }

        if field.is_some() {
            // this.x.field
            if var.name == node_fieldless {
                let mut kind = var.kind.clone();
                kind.flatten();
                return match lookup::class::in_env(&kind, current, kinds) {
                    Ok(f) => {
                        let mut result = None;

                        for cls_field in &f.fields {
                            if cls_field.name != field.clone().unwrap() {
                                continue;
                            }

                            result = Some(cls_field.to_variable().kind.clone());
                        }

                        match result {
                            Some(k) => {
                                let kind =
                                    ClassOrInterfaceEnvironment::new(k, ClassOrInterface::CLASS);
                                Ok(Type::new(kind))
                            }
                            None => Err(format!("could not find(2) field {} on {}", node, f)),
                        }
                    }
                    Err(e) => Err(e),
                };
            }

            // x.field
            if var.name.children.len() == 3 &&
               var.name.children[0].clone().token.kind == TokenKind::This &&
               var.name.children[2] == node_fieldless {
                let mut kind = var.kind.clone();
                kind.flatten();
                return match lookup::class::in_env(&kind, current, kinds) {
                    Ok(f) => {
                        let mut result = None;

                        for cls_field in &f.fields {
                            if cls_field.name != field.clone().unwrap() {
                                continue;
                            }

                            result = Some(cls_field.to_variable().kind.clone());
                        }

                        match result {
                            Some(k) => {
                                let kind =
                                    ClassOrInterfaceEnvironment::new(k, ClassOrInterface::CLASS);
                                Ok(Type::new(kind))
                            }
                            None => Err(format!("could not find(3) field {} on {}", node, f)),
                        }
                    }
                    Err(e) => Err(e),
                };
            }

            // TODO: this should nest arbitrarily...
            // other.x.field
        }
    }

    loop {
        if field.is_none() {
            break;
        }

        let cls = match lookup::class::in_env(&node_fieldless, current, kinds) {
            Ok(c) => c,
            Err(_) => break,
        };

        let field = field.unwrap();
        for f in &cls.fields {
            if &f.name == &field {
                match lookup::class::in_env(&f.to_variable().kind, &cls, kinds) {
                    Ok(cls) => return Ok(Type::new(cls)),
                    Err(_) => (),
                }
            }
        }

        break;
    }

    match lookup::class::in_env(&node, current, kinds) {
        Ok(f) => Ok(Type::new(f)),
        Err(e) => Err(e),
    }
}
