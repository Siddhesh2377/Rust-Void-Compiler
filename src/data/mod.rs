pub struct Function {
    pub name: String,
    pub body: String,
}

impl Function {
    pub fn new(name: String, body: String) -> Function {
        Function { name, body }
    }
}

pub struct MethodCall {
    pub parent_method: String,
    pub name: String,
    pub args: Vec<Argument>,
}

impl MethodCall {
    pub fn new(parent_method: String, name: String, args: Vec<Argument>) -> MethodCall {
        MethodCall { parent_method, name, args }
    }
}

pub struct Argument {
    pub value: String,
    pub arg_type: String,
}

impl Argument {
    pub fn new(value: String, arg_type: String) -> Argument {
        Argument { value, arg_type }
    }
}
