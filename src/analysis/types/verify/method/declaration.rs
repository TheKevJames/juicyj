use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::lookup;
use analysis::types::obj::Type;
use analysis::types::resolve;
use analysis::types::verify;
use scanner::ASTNode;
use scanner::TokenKind;

pub fn go(node: &ASTNode,
          kinds: &Vec<ClassOrInterfaceEnvironment>,
          current: &ClassOrInterfaceEnvironment,
          globals: &Vec<VariableEnvironment>,
          locals: &mut Vec<VariableEnvironment>)
          -> Result<(), String> {
    let new = VariableEnvironment::new(node.clone());
    // TODO: chain
    for global in globals.clone() {
        if global.name != new.name {
            continue;
        }

        return Err(format!("cannot declare variable {} with conflict in outer scope",
                           new.name));
    }
    for local in locals.clone() {
        if local.name != new.name {
            continue;
        }

        return Err(format!("cannot declare variable {} with conflict in local scope",
                           new.name));
    }

    match node.children[1].clone().token.kind {
        TokenKind::Assignment => {
            let mut rvalue = node.children[1].clone().children[1].clone();
            match verify::method::statement::nonblock(&mut rvalue,
                                                      current,
                                                      kinds,
                                                      globals,
                                                      locals) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }

            let lhs = match lookup::class::in_env(&new.kind, current, kinds) {
                Ok(l) => Type::new(l),
                Err(e) => return Err(e),
            };

            let mut block_globals = globals.clone();
            block_globals.extend(locals.clone());
            let rhs = match resolve::expression::go(&rvalue, current, kinds, &block_globals) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };

            match lhs.assign(&rhs, current, kinds) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        _ => (),
    }

    locals.push(new.clone());
    verify::class::resolveable(&new.kind, current, kinds)
}