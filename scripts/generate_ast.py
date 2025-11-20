

def define_ast(output_dir, file_name, types):
    path = f"{output_dir}/{file_name}.rs"
    with open(path, 'w') as f:
        f.write("enum Expr {\n")
        for type_ in types:
            f.write(f"  {type_[0]} " + "{" + f"{type_[1]}" + "},\n")
        f.write("}\n")


output_dir = "./src"
define_ast(output_dir, "expr", [
    ("Binary", "left: Expr, operator: Token, right: Expr"),
    ("Grouping", "expression: Expr"),
    ("Literal", "value: LiteralType"),
    ("Unary", "operator: Token, right: Expr"),

])
