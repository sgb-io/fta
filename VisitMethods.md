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

Not yet implemented:

visit_ts_as_expr
visit_ts_call_signature_decl
visit_ts_conditional_type
visit_ts_const_assertion
visit_ts_construct_signature_decl
visit_ts_constructor_type

visit_ts_entity_name
visit_ts_enum_decl
visit_ts_enum_member
visit_ts_enum_member_id
visit_ts_enum_members
visit_ts_export_assignment

visit_ts_expr_with_type_args
visit_ts_expr_with_type_args_vec
visit_ts_external_module_ref
visit_ts_fn_or_constructor_type
visit_ts_fn_param
visit_ts_fn_params

visit_ts_fn_type
visit_ts_getter_signature
visit_ts_import_equals_decl
visit_ts_import_type
visit_ts_index_signature
visit_ts_indexed_access_type
visit_ts_infer_type

visit_ts_instantiation
visit_ts_interface_body
visit_ts_interface_decl
visit_ts_intersection_type
visit_ts_keyword_type
visit_ts_keyword_type_kind
visit_ts_lit

visit_ts_lit_type
visit_ts_mapped_type
visit_ts_method_signature
visit_ts_module_block
visit_ts_module_decl
visit_ts_module_name

visit_ts_module_ref
visit_ts_namespace_body
visit_ts_namespace_decl
visit_ts_namespace_export_decl
visit_ts_non_null_expr
visit_ts_optional_type
visit_ts_param_prop

visit_ts_param_prop_param
visit_ts_parenthesized_type
visit_ts_property_signature
visit_ts_qualified_name
visit_ts_rest_type
visit_ts_satisfies_expr

visit_ts_setter_signature
visit_ts_this_type
visit_ts_this_type_or_ident
visit_ts_tpl_lit_type
visit_ts_tuple_element
visit_ts_tuple_elements

visit_ts_tuple_type
visit_ts_type
visit_ts_type_alias_decl
visit_ts_type_ann
visit_ts_type_assertion
visit_ts_type_element

visit_ts_type_elements
visit_ts_type_lit
visit_ts_type_operator
visit_ts_type_operator_op
visit_ts_type_param
visit_ts_type_param_decl

visit_ts_type_param_instantiation
visit_ts_type_params
visit_ts_type_predicate
visit_ts_type_query
visit_ts_type_query_expr
visit_ts_type_ref

visit_ts_types
visit_ts_union_or_intersection_type
visit_ts_union_type
visit_unary_op
visit_update_op
visit_var_decl

visit_var_decl_kind
visit_var_decl_or_expr
visit_var_decl_or_pat
visit_var_declarators
visit_with_stmt

Struggling with / skipped for now:

visit_opt_call
visit_opt_catch_clause
visit_opt_chain_base
visit_opt_chain_expr
visit_opt_expr
visit_opt_expr_or_spread
visit_opt_expr_or_spreads
visit_opt_ident
visit_opt_jsx_attr_value
visit_opt_jsx_closing_element
visit_opt_module_export_name
visit_opt_module_items
visit_opt_object_lit
visit_opt_pat
visit_opt_span
visit_opt_stmt
visit_opt_str
visit_opt_true_plus_minus
visit_opt_ts_entity_name
visit_opt_ts_namespace_body
visit_opt_ts_type
visit_opt_ts_type_ann
visit_opt_ts_type_param_decl
visit_opt_ts_type_param_instantiation
visit_opt_var_decl_or_expr
visit_opt_vec_expr_or_spreads
visit_opt_vec_pats
visit_param_or_ts_param_prop
