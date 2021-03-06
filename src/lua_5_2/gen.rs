
use super::ast::*;

pub fn gen_code(ast : Vec<Statement>) -> String {
    let mut ret : Vec<String> = vec![]; 
    let tab = 0;

    for statement in ast {
        let v = gen_statement(statement, tab);
        ret.push(v);
    }

    ret.into_iter().map(|v| format!("{}\n", v)).collect::<String>()
}

fn gen_statement(statement : Statement, tab : usize) -> String {
    match statement {
        Statement::LocalVarDeclare(name) => format!("{}local {}", " ".repeat(tab * 4), name),
        Statement::Return(es) => {
            let exprs = es.into_iter().map(|e| gen_expr(e, tab)).collect::<Vec<String>>();

            format!("{}return {}", " ".repeat(tab * 4), exprs.join(", "))
        },
        Statement::Break => format!("{}break", " ".repeat(tab * 4)),
        Statement::If { mut if_statements, else_statement } => {
            let first = if_statements.remove(0);

            let first_text = gen_if(first, tab);

            let elseifs = if_statements
                .into_iter()
                .map(|s| gen_if(s, tab))
                .map(|s| format!("{}elseif {}\n", " ".repeat(tab * 4), s))
                .collect::<String>();

            if else_statement.len() == 0 {
                format!("{}if {}\n{}{}end",
                    " ".repeat(tab * 4),
                    first_text,
                    elseifs,
                    " ".repeat(tab * 4))
            }
            else {
                let else_statements = else_statement
                    .into_iter()
                    .map(|s| gen_statement(s, tab + 1))
                    .map(|s| format!("{}\n", s))
                    .collect::<String>();

                format!("{}if {}\n{}{}else\n{}{}end",
                    " ".repeat(tab * 4),
                    first_text,
                    elseifs,
                    " ".repeat(tab * 4),
                    else_statements,
                    " ".repeat(tab * 4))
            }
        },
        Statement::AssignVar { vars, exprs } => {
            format!( "{}{} = {}\n"
                   , " ".repeat(tab * 4)
                   , vars.join(", ")
                   , exprs.into_iter()
                          .map(|e| gen_expr(e, tab))
                          .collect::<Vec<String>>()
                          .join(", ")
                   )
        },
        Statement::AssignListAccess { target, index, new_value } => {
            let list_expr = gen_expr(target, tab);
            let index_expr = gen_expr(index, tab);
            let value_expr = gen_expr(new_value, tab);
            format!( "{}{}[ {} ] = {}"
                   , " ".repeat(tab * 4)
                   , list_expr
                   , index_expr
                   , value_expr
                   )
        },
        Statement::AssignTableAccess { target, slot, new_value } => {
            let table_expr = gen_expr(target, tab);
            let value_expr = gen_expr(new_value, tab);
            format!( "{}{}.{} = {}"
                   , " ".repeat(tab * 4)
                   , table_expr
                   , slot
                   , value_expr
                   )
        },
        Statement::While { test, statements } => {
            let test_expr = gen_expr(test, tab);

            let statements_text = statements 
                .into_iter()
                .map(|s| gen_statement(s, tab + 1))
                .map(|s| format!("{}\n", s))
                .collect::<String>();
          
            format!( "{}while {} do\n{}{}end"
                   , " ".repeat(tab * 4)
                   , test_expr
                   , statements_text
                   , " ".repeat(tab * 4)
                   )
        },
        Statement::Repeat { test, statements } => {
            let test_expr = gen_expr(test, tab);

            let statements_text = statements 
                .into_iter()
                .map(|s| gen_statement(s, tab + 1))
                .map(|s| format!("{}\n", s))
                .collect::<String>();
          
            format!( "{}repeat\n{}{}until {}"
                   , " ".repeat(tab * 4)
                   , statements_text
                   , " ".repeat(tab * 4)
                   , test_expr
                   )
        },
        Statement::For { vars, iterator, statements } => {
            let vars_text = vars.join("\n");

            let iterator_text = gen_expr(iterator, tab);

            let statements_text = statements 
                .into_iter()
                .map(|s| gen_statement(s, tab + 1))
                .map(|s| format!("{}\n", s))
                .collect::<String>();

            format!( "{}for {} in {} do\n{}{}end"
                   , " ".repeat(tab * 4)
                   , vars_text
                   , iterator_text
                   , statements_text
                   , " ".repeat(tab * 4)
                   )
        },
        Statement::ForI { var, start, end, increment, statements } => {
            let start_text = gen_expr(start, tab);
            let end_text = gen_expr(end, tab);
            
            let increment_text = match increment {
                Some(i) => gen_expr(i, tab),
                None => "1".to_string(),
            };

            let statements_text = statements 
                .into_iter()
                .map(|s| gen_statement(s, tab + 1))
                .map(|s| format!("{}\n", s))
                .collect::<String>();

            format!( "{}for {} = {}, {}, {} do\n{}{}end"
                   , " ".repeat(tab * 4)
                   , var
                   , start_text
                   , end_text
                   , increment_text
                   , statements_text
                   , " ".repeat(tab * 4)
                   )
        },
        Statement::FunCall { fun, params } => {
            let fun_text = gen_expr(fun, tab);
            let params_text = params
                .into_iter()
                .map(|s| gen_expr(s, tab))
                .collect::<Vec<String>>()
                .join(", ");

            format!( "{}{}( {} )"
                   , " ".repeat(tab * 4)
                   , fun_text
                   , params_text
                   )
        },
        Statement::CallSystemFun { fun, params } => {
            let params_text = params
                .into_iter()
                .map(|s| gen_expr(s, tab))
                .collect::<Vec<String>>()
                .join(", ");

            format!( "{}{}( {} )"
                   , " ".repeat(tab * 4)
                   , fun
                   , params_text
                   )
        },
    }
}

fn gen_if(if_statement : If, tab : usize) -> String {
    let test = gen_expr(if_statement.test, tab); 
    let mut statements = vec![];
    for s in if_statement.statements {
        let st = gen_statement(s, tab + 1);
        statements.push(st);
    }

    let statements_text = statements.join("\n");

    format!( "{} then\n{}", test, statements_text )
}

fn gen_expr(expr : Expr, tab : usize) -> String {
    match expr {
        Expr::Nil => "nil".to_string(),
        Expr::Number(s) => s,
        Expr::String(s) => s,
        Expr::Bool(true) => "true".to_string(),
        Expr::Bool(false) => "false".to_string(),
        Expr::Var(s) => s,
        Expr::TableCons(inline_table_assign) => {
            let assigns = inline_table_assign
                .into_iter()
                .map(|a| format!( " [\"{}\"] = {} "
                                , a.key
                                , gen_expr(a.value, tab)
                                ))
                .collect::<Vec<String>>()
                .join("; ");
            
            format!( "{{ {} }}", assigns ) 
        },
        Expr::TableAccess { expr, slot } => {
            let expr_text = gen_expr(*expr, tab);
            format!( "{}.{}", expr_text, slot )
        },
        Expr::ListCons(exprs) => {
            let exprs_text = exprs
                .into_iter()
                .map(|e| gen_expr(e, tab))
                .collect::<Vec<String>>()
                .join(", ");
            format!( "{{ {} }}", exprs_text )
        },
        Expr::ListAccess { expr, index } => {
            let expr_text = gen_expr(*expr, tab);
            let index_text = gen_expr(*index, tab);
            format!( "{}[ {} ]"
                   , expr_text
                   , index_text
                   )
        },
        Expr::FunCall { fun, params } => {
            let fun_text = gen_expr(*fun, tab);
            let params_text = params
                .into_iter()
                .map(|s| gen_expr(s, tab))
                .collect::<Vec<String>>()
                .join(", ");

            format!( "{}( {} )"
                   , fun_text
                   , params_text
                   )
        },
        Expr::Lambda { params, statements } => {
            let params_text = params.join(", ");
            let statements_text = statements
                .into_iter()
                .map(|s| gen_statement(s, tab + 1))
                .map(|s| format!("{}\n", s))
                .collect::<String>();

            format!( "function ({}) {}{}end"
                   , params_text
                   , statements_text
                   , " ".repeat(tab * 4)
                   )
        },
        Expr::Paren(expr) => {
            let expr_text = gen_expr(*expr, tab);

            format!( "( {} )", expr_text )
        },
        Expr::CallSystemFun { fun, params } => {
            let params_text = params
                .into_iter()
                .map(|s| gen_expr(s, tab))
                .collect::<Vec<String>>()
                .join(", ");

            format!( "{}( {} )"
                   , fun
                   , params_text
                   )
        },
        Expr::CallBinFun { fun, a, b } => {
            let a_text = gen_expr(*a, tab);
            let b_text = gen_expr(*b, tab);

            format!( "{} {} {}"
                   , a_text
                   , fun
                   , b_text
                   )
        },
        Expr::CallUniFun { fun, a } => {
            let a_text = gen_expr(*a, tab);

            format!( "{}{}"
                   , a_text
                   , fun
                   )
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_handle_if() {
        let if_statements = vec! [
                                    If { test: Expr::Number("0".to_string())
                                       , statements: vec! [ Statement::Break 
                                                          , Statement::Break
                                                          ]
                                       }
                                 ];

        let ast = vec! [ Statement::If { if_statements, else_statement: vec![] } ];
                                         
        let output = gen_code( ast );

        assert_eq!( output, 
r#"if 0 then
    break
    break
end
"#);
    }

    #[test]
    fn should_handle_if_with_elseif() {
        let if_statements = vec! [
            If { test: Expr::Number("0".to_string())
               , statements: vec! [ Statement::Break 
                                  , Statement::Break
                                  ]
               },
            If { test: Expr::Number("1".to_string())
               , statements: vec! [ Statement::Break, Statement::Break ]
               }
        ];


        let ast = vec! [ Statement::If { if_statements, else_statement: vec![] } ];
                                         
        let output = gen_code( ast );

        assert_eq!( output, 
r#"if 0 then
    break
    break
elseif 1 then
    break
    break
end
"#);
    }

    #[test]
    fn should_handle_if_with_elseif_with_elseif() {

        let if_statements = vec! [
            If { test: Expr::Number("0".to_string())
               , statements: vec! [ Statement::Break 
                                  , Statement::Break
                                  ]
               },
            If { test: Expr::Number("1".to_string())
               , statements: vec! [ Statement::Break, Statement::Break ]
               },
            If { test: Expr::Number("2".to_string())
               , statements: vec! [ Statement::Break, Statement::Break ]
               }
        ];

        let ast = vec! [ Statement::If { if_statements, else_statement: vec![] } ];
                                         
        let output = gen_code( ast );

        assert_eq!( output, 
r#"if 0 then
    break
    break
elseif 1 then
    break
    break
elseif 2 then
    break
    break
end
"#);
    }

    #[test]
    fn should_handle_if_with_elseif_with_elseif_with_else() {

        let if_statements = vec! [
            If { test: Expr::Number("0".to_string())
               , statements: vec! [ Statement::Break 
                                  , Statement::Break
                                  ]
               },
            If { test: Expr::Number("1".to_string())
               , statements: vec! [ Statement::Break, Statement::Break ]
               },
            If { test: Expr::Number("2".to_string())
               , statements: vec! [ Statement::Break, Statement::Break ]
               }
        ];

        let ast = vec! [ Statement::If { if_statements, else_statement: vec![ Statement::Break, Statement::Break ] } ];

        let output = gen_code( ast );

        assert_eq!( output, 
r#"if 0 then
    break
    break
elseif 1 then
    break
    break
elseif 2 then
    break
    break
else
    break
    break
end
"#);
    }

    #[test]
    fn should_handle_if_with_else() {

        let if_statements = vec! [
            If { test: Expr::Number("0".to_string())
               , statements: vec! [ Statement::Break 
                                  , Statement::Break
                                  ]
               },
        ];

        let ast = vec! [ Statement::If { if_statements, else_statement: vec![ Statement::Break, Statement::Break ] } ];
                                         
        let output = gen_code( ast );

        assert_eq!( output, 
r#"if 0 then
    break
    break
else
    break
    break
end
"#);
    }

    #[test]
    fn should_handle_if_with_nested_if() {

        let if_statements = vec! [
            If { test: Expr::Number("0".to_string())
               , statements: vec![ Statement::If { 
                                       if_statements: vec![ If { test: Expr::Number("1".to_string()),
                                                                 statements: vec![ Statement::Break, 
                                                                                   Statement::Break
                                                                                 ] 
                                                               }
                                                          ],
                                       else_statement: vec![]
                                                 }
                                 ]
               }
        ];

        let ast = vec! [ Statement::If { if_statements, else_statement: vec![] } ];

        let output = gen_code( ast );

        assert_eq!( output, 
r#"if 0 then
    if 1 then
        break
        break
    end
end
"#);
    }
}
