use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Variable {
    Stack(i32),       // offset
    Register(String), // register name
    Global(String),   // .data label
}

pub struct Allocator {
    next_stack_offset: i32,
    used_registers: Vec<String>,
    scopes: Vec<HashMap<String, Variable>>,
}

impl Allocator {
    pub fn new(registers: &[&str], global_vars: &HashMap<String, Variable>) -> Self {
        let scopes = vec![global_vars.clone()];
        Self {
            next_stack_offset: 0,
            used_registers: registers.iter().map(|s| s.to_string()).collect(),
            scopes,
        }
    }

    pub fn allocate(&mut self, name: String, size: i32) -> Variable {
        if size == 4 {
            if let Some(var) = self.try_allocate_register(name.clone()) {
                return var;
            }
        }
        self.allocate_stack(name, size)
    }

    fn try_allocate_register(&mut self, name: String) -> Option<Variable> {
        if self.used_registers.is_empty() {
            return None;
        }
        let register = self.used_registers.pop().unwrap();
        let var = Variable::Register(register);
        self.scopes.last_mut().unwrap().insert(name, var.clone());
        Some(var)
    }

    fn allocate_stack(&mut self, name: String, size: i32) -> Variable {
        self.next_stack_offset -= size;
        let var = Variable::Stack(self.next_stack_offset);
        self.scopes.last_mut().unwrap().insert(name, var.clone());
        var
    }

    pub fn get(&self, name: &str) -> Option<&Variable> {
        for scope in self.scopes.iter().rev() {
            if let Some(var) = scope.get(name) {
                return Some(var);
            }
        }
        None
    }

    pub fn total_stack_size(&self) -> i32 {
        self.next_stack_offset.abs()
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }
}
