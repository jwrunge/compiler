use std::collections::HashMap;

pub struct ConstArgs {
    reg: String,
    val: i32,
}

pub struct AutoArgs {
    reg: String,
    func: String,
    reg_arg: String,
}

pub struct AsmMap {
    const_args: Option<Vec<ConstArgs>>,
    user_arg_regs: Option<Vec<String>>,
    auto_args: Option<AutoArgs>,
}

fn get_files_list() {

}

pub fn get_asm_maps() -> HashMap<String, HashMap<String, AsmMap>> {
    let mut arches: HashMap<String, HashMap<String, AsmMap>> = HashMap::new();
    let mut oses: HashMap<String, AsmMap> = HashMap::new();
    let m = AsmMap {
        const_args: None,
        user_arg_regs: None,
        auto_args: None,
    };
    oses.insert("linux".to_string(), m);
    arches.insert("x86_64".to_string(), oses);
    arches
}