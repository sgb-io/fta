Methods to implement for the `swc_ecma_visit::Visit` trait

| Function Name                    | Description                                                                                                          |
| -------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| visit_expr                       | Visits expressions, which can contain both operands (e.g., literals, identifiers) and operators (e.g., binary)       |
| visit_fn_decl                    | Visits function declarations, which can contain the 'function' keyword as an operator and function names as operands |
| visit_class_decl                 | Visits class declarations, which can contain the 'class' keyword as an operator and class names as operands          |
| visit_class_method               | Visits class methods, which can contain method names as operands and introduce 'method' as an operator               |
| visit_var_declarator             | Visits variable declarators, which can contain variable names as operands and assignment operators                   |
| visit_pat                        | Visits patterns, which can contain variable names as operands                                                        |
| visit_arrow_expr                 | Visits arrow expressions, which can contain the '=>' operator                                                        |
| visit_update_expr                | Visits update expressions (e.g., ++, --), which can contain the update operators                                     |
| visit_assign_expr                | Visits assignment expressions (e.g., +=, -=), which can contain the assignment operators                             |
| visit_unary_expr                 | Visits unary expressions (e.g., !, ~, typeof), which can contain unary operators                                     |
| visit_member_expr                | Visits member expressions (e.g., obj.prop, arr[0]), which can contain the '.' or '[]' operators                      |
| visit_new_expr                   | Visits 'new' expressions, which can contain the 'new' operator                                                       |
| visit_call_expr                  | Visits call expressions, which can contain the '()' operator                                                         |
| visit_conditional_expr           | Visits conditional expressions (e.g., a ? b : c), which can contain the '?' and ':' operators                        |
| visit_template_literal           | Visits template literals, which can contain expressions as operands                                                  |
| visit_for_stmt                   | Visits 'for' statements, which can contain the 'for' keyword as an operator                                          |
| visit_for_in_stmt                | Visits 'for-in' statements, which can contain the 'for' and 'in' keywords as operators                               |
| visit_for_of_stmt                | Visits 'for-of' statements, which can contain the 'for' and 'of' keywords as operators                               |
| visit_while_stmt                 | Visits 'while' statements, which can contain the 'while' keyword as an operator                                      |
| visit_do_while_stmt              | Visits 'do-while' statements, which can contain the 'do' and 'while' keywords as operators                           |
| visit_if_stmt                    | Visits 'if' statements, which can contain the 'if' and 'else' keywords as operators                                  |
| visit_switch_stmt                | Visits 'switch' statements, which can contain the 'switch' keyword as an operator                                    |
| visit_break_stmt                 | Visits 'break' statements, which can contain the 'break' keyword as an operator                                      |
| visit_continue_stmt              | Visits 'continue' statements, which can contain the 'continue' keyword as an operator                                |
| visit_return_stmt                | Visits 'return' statements, which can contain the 'return' keyword as an operator                                    |
| visit_throw_stmt                 | Visits 'throw' statements, which can contain the 'throw' keyword as an operator                                      |
| visit_tagged_tpl_expr            | Visits tagged template expressions, which can contain expressions as operands and template strings                   |
| visit_await_expr                 | Visits 'await' expressions, which can contain the 'await' keyword as an operator                                     |
| visit_yield_expr                 | Visits 'yield' expressions, which can contain the 'yield' keyword as an operator                                     |
| visit_import_decl                | Visits 'import' declarations, which can contain the 'import' keyword as an operator                                  |
| visit_export_decl                | Visits 'export' declarations, which can contain the 'export' keyword as an operator                                  |
| visit_export_default_decl        | Visits 'export default' declarations, which can contain the 'export default' keywords as operators                   |
| visit_export_named_specifier     | Visits named export specifiers, which can contain identifiers as operands                                            |
| visit_export_namespace_specifier | Visits namespace export specifiers, which can contain identifiers as operands                                        |
| visit_import_named_specifier     | Visits named import specifiers, which can contain identifiers as operands                                            |
| visit_import_default_specifier   | Visits default import specifiers, which can contain identifiers as operands                                          |
| visit_import_namespace_specifier | Visits namespace import specifiers, which can contain identifiers as operands                                        |
| visit_try_stmt                   | Visits 'try-catch' statements, which can contain the 'try', 'catch', and 'finally' keywords as operators             |

Also

visit_bin_expr: Binary expressions contain binary operators and two operands.
visit_unary_expr: Unary expressions contain unary operators and one operand.
visit_assign_expr: Assignment expressions contain an assignment operator and two operands.
visit_update_expr: Update expressions (e.g., x++, x--) contain update operators and one operand.
visit_member_expr: Member expressions (e.g., obj.prop) contain an implicit member access operator and two operands.

visit_conditional_expr: Conditional expressions (e.g., a ? b : c) contain two implicit operators (conditional and alternate) and three operands.
visit_call_expr: Call expressions (e.g., func(arg1, arg2)) contain an implicit call operator and a variable number of operands.
visit_new_expr: New expressions (e.g., new MyClass(arg1, arg2)) contain an implicit constructor call operator and a variable number of operands.
visit_ident: Identifiers represent operands (e.g., variable names, function names).
visit_lit: Literals represent operands (e.g., numeric, string, boolean, null).

visit_arrow_expr: Arrow expressions (e.g., (a, b) => a + b) contain an implicit function definition operator.
visit_tpl_lit: Template literals (e.g., `Hello, ${name}!`) represent operands and may contain expressions.
visit_tagged_tpl_lit: Tagged template literals (e.g., tagHello, ${name}!``) represent a call operator and a variable number of operands.
visit_spread_element: Spread elements (e.g., ...arr) contain an implicit spread operator.
visit_ts_non_null_expr: Non-null expressions (e.g., x!) contain an implicit non-null assertion operator.

visit_ts_type_assertion: TypeScript type assertion expressions (e.g., <string>x) contain an implicit type assertion operator.
visit_ts_as_expr: TypeScript 'as' type assertion expressions (e.g., x as string) contain an implicit type assertion operator.
visit_ts_type_operator: TypeScript type operators (e.g., keyof T, readonly T[], T | null) contain type operators.
visit_ts_qualified_name: TypeScript qualified names (e.g., Namespace.Type) contain an implicit namespace access operator.
visit_ts_mapped_type: TypeScript mapped types (e.g., { [P in keyof T]: T[P] }) contain an implicit mapped type operator.
visit_ts_indexed_access_type: TypeScript indexed access types (e.g., T['property']) contain an implicit indexed access operator.
