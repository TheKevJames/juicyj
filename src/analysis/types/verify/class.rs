use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::types::lookup;
use scanner::ASTNode;

pub fn resolveable(name: &ASTNode,
                   current: &ClassOrInterfaceEnvironment,
                   kinds: &Vec<ClassOrInterfaceEnvironment>)
                   -> Result<(), String> {
    let cls = match lookup::class::in_env(name, current, kinds) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    if let Some(l) = cls.name.token.lexeme {
        if l == "ArrayType" {
            let cls = lookup::class::in_env(&cls.name.children[0], current, kinds);
            if cls.is_err() {
                return Err(cls.unwrap_err());
            }
        }
    }

    Ok(())
}
