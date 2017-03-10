use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::types::lookup;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref NAME: Token = Token::new(TokenKind::NonTerminal, Some("Name"));
}
const PRIMITIVES: [TokenKind; 7] = [TokenKind::Boolean,
                                    TokenKind::Byte,
                                    TokenKind::Char,
                                    TokenKind::Int,
                                    TokenKind::Null,
                                    TokenKind::Short,
                                    TokenKind::Void];

pub fn canonical(canonical: &ASTNode,
                 current: &ClassOrInterfaceEnvironment,
                 kinds: &Vec<ClassOrInterfaceEnvironment>)
                 -> Result<(), String> {
    if !canonical.children.is_empty() && PRIMITIVES.contains(&canonical.children[0].token.kind) {
        return Err(format!("strict prefix of {:?} resolves to primitive type",
                           canonical));
    }

    let mut prefix = Vec::new();
    for (idx, child) in canonical.children.iter().enumerate() {
        prefix.push(child.clone());
        if idx % 2 != 0 {
            // canonical is "a.b.c". Prefixes should not have trailing Dot.
            continue;
        }

        let name = ASTNode {
            token: NAME.clone(),
            children: prefix.clone(),
        };
        if &name == canonical {
            break;
        }

        // TODO: remove recursive references
        match lookup::class::lookup_step0_canonical(&name, current, kinds) {
            Some(Ok(_)) => return Err(format!("strict prefix {} resolves to canonical type", name)),
            _ => (),
        }

        match lookup::class::lookup_step3_enclosing_package(&name, current, kinds) {
            Some(Ok(_)) => return Err(format!("strict prefix {} resolves to local type", name)),
            _ => (),
        }
    }

    Ok(())
}

pub fn package(canonical: &ASTNode,
               current: &ClassOrInterfaceEnvironment,
               kinds: &Vec<ClassOrInterfaceEnvironment>)
               -> Result<(), String> {
    if !canonical.children.is_empty() && PRIMITIVES.contains(&canonical.children[0].token.kind) {
        return Err(format!("strict prefix of {:?} resolves to primitive type",
                           canonical));
    }

    let mut prefix = Vec::new();
    for (idx, child) in canonical.children.iter().enumerate() {
        prefix.push(child.clone());
        if idx % 2 != 0 {
            // canonical is "a.b.c". Prefixes should not have trailing Dot.
            continue;
        }

        let name = ASTNode {
            token: NAME.clone(),
            children: prefix.clone(),
        };
        if &name == canonical {
            break;
        }

        // TODO: this should recurse to self, not verify::prefixes::canonical
        match lookup::class::lookup_step0_canonical(&name, current, kinds) {
            Some(Ok(_)) => return Err(format!("strict prefix {} resolves to canonical type", name)),
            _ => (),
        }
    }

    Ok(())
}
