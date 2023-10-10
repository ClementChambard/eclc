pub fn gen_table() -> ProductionTable {
    let mut table = vec![];
    table.push(ProductionTableEntry::new(
        "Comma_sep_str_opt",
        "str",
        vec![Symbol::T("str".to_owned()), Symbol::NT("Comma_sep_str2".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Comma_sep_str_opt",
        "rb",
        vec![]
    ));
    table.push(ProductionTableEntry::new(
        "Comma_sep_str2",
        ",",
        vec![Symbol::T(",".to_owned()), Symbol::T("str".to_owned()), Symbol::NT("Comma_sep_str2".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Comma_sep_str2",
        "rb",
        vec![]
    ));
    table.push(ProductionTableEntry::new(
        "Param_list",
        "int",
        vec![Symbol::NT("Expr".to_owned()), Symbol::NT("Param_list2".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Param_list",
        "float",
        vec![Symbol::NT("Expr".to_owned()), Symbol::NT("Param_list2".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Param_list",
        "str",
        vec![Symbol::NT("Expr".to_owned()), Symbol::NT("Param_list2".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Param_list",
        "[",
        vec![Symbol::NT("Expr".to_owned()), Symbol::NT("Param_list2".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Param_list",
        "id",
        vec![Symbol::NT("Expr".to_owned()), Symbol::NT("Param_list2".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Param_list",
        ")",
        vec![]
    ));
    table.push(ProductionTableEntry::new(
        "Param_list2",
        ",",
        vec![Symbol::T(",".to_owned()), Symbol::NT("Expr".to_owned()), Symbol::NT("Param_list2".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Param_list2",
        ")",
        vec![]
    ));
    table.push(ProductionTableEntry::new(
        "Ecl",
        "kw_ecli",
        vec![Symbol::NT("Ecli".to_owned()), Symbol::NT("Anmi".to_owned()), Symbol::NT("SubList".to_owned()), Symbol::T("EOF".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Ecli",
        "kw_ecli",
        vec![Symbol::T("kw_ecli".to_owned()), Symbol::T("lb".to_owned()), Symbol::NT("Comma_sep_str_opt".to_owned()), Symbol::T("rb".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Anmi",
        "kw_anmi",
        vec![Symbol::T("kw_anmi".to_owned()), Symbol::T("lb".to_owned()), Symbol::NT("Comma_sep_str_opt".to_owned()), Symbol::T("rb".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "SubList",
        "kw_sub",
        vec![Symbol::NT("Sub".to_owned()), Symbol::NT("SubList".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "SubList",
        "EOF",
        vec![]
    ));
    table.push(ProductionTableEntry::new(
        "Sub",
        "kw_sub",
        vec![Symbol::T("kw_sub".to_owned()), Symbol::T("id".to_owned()), Symbol::T("(".to_owned()), Symbol::T(")".to_owned()), Symbol::NT("BlocInstr".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "BlocInstr",
        "lb",
        vec![Symbol::T("lb".to_owned()), Symbol::NT("InstrList".to_owned()), Symbol::T("rb".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "InstrList",
        "lb",
        vec![Symbol::NT("Instr".to_owned()), Symbol::NT("InstrList".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "InstrList",
        "id",
        vec![Symbol::NT("Instr".to_owned()), Symbol::NT("InstrList".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "InstrList",
        "int",
        vec![Symbol::NT("Instr".to_owned()), Symbol::NT("InstrList".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "InstrList",
        "rb",
        vec![]
    ));
    table.push(ProductionTableEntry::new(
        "Instr",
        "id",
        vec![Symbol::T("id".to_owned()), Symbol::NT("Instr_sub".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Instr",
        "lb",
        vec![Symbol::NT("BlocInstr".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Instr",
        "int",
        vec![Symbol::T("int".to_owned()), Symbol::T(":".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Instr_sub",
        "(",
        vec![Symbol::T("(".to_owned()), Symbol::NT("Param_list".to_owned()), Symbol::T(")".to_owned()), Symbol::T(";".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Instr_sub",
        ":",
        vec![Symbol::T(":".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Expr",
        "int",
        vec![Symbol::NT("Expr2".to_owned()), Symbol::NT("Exprp".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Expr",
        "id",
        vec![Symbol::NT("Expr2".to_owned()), Symbol::NT("Exprp".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Expr",
        "float",
        vec![Symbol::NT("Expr2".to_owned()), Symbol::NT("Exprp".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Expr",
        "str",
        vec![Symbol::NT("Expr2".to_owned()), Symbol::NT("Exprp".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Expr",
        "[",
        vec![Symbol::NT("Expr2".to_owned()), Symbol::NT("Exprp".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Exprp",
        "+",
        vec![Symbol::T("+".to_owned()), Symbol::NT("Expr2".to_owned()), Symbol::NT("Exprp".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Exprp",
        ",",
        vec![]
    ));
    table.push(ProductionTableEntry::new(
        "Exprp",
        ")",
        vec![]
    ));
    table.push(ProductionTableEntry::new(
        "Expr2",
        "int",
        vec![Symbol::T("int".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Expr2",
        "float",
        vec![Symbol::T("float".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Expr2",
        "str",
        vec![Symbol::T("str".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Expr2",
        "id",
        vec![Symbol::T("id".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "Expr2",
        "[",
        vec![Symbol::T("[".to_owned()), Symbol::NT("VarExpr".to_owned()), Symbol::T("]".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "VarExpr",
        "int",
        vec![Symbol::T("int".to_owned())]
    ));
    table.push(ProductionTableEntry::new(
        "VarExpr",
        "float",
        vec![Symbol::T("float".to_owned())]
    ));
    table
}
