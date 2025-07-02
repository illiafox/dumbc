use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Variable {
    Stack(i32), // offset
    Register(String),
}

#[derive(Clone)]
pub struct Allocator {
    next_stack_offset: i32,
    used_registers: Vec<String>,
    vars: HashMap<String, Variable>,
}

impl Allocator {
    pub fn new(registers: Vec<&str>) -> Self {
        Self {
            next_stack_offset: 0,
            used_registers: registers.iter().cloned().map(str::to_string).collect(),
            vars: HashMap::new(),
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

        self.vars.insert(name, var.clone());
        Some(var)
    }

    fn allocate_stack(&mut self, name: String, size: i32) -> Variable {
        self.next_stack_offset -= size; // alignment
        let var = Variable::Stack(self.next_stack_offset);
        self.vars.insert(name, var.clone());
        var
    }

    pub fn get(&self, name: &str) -> Option<&Variable> {
        self.vars.get(name)
    }

    pub fn total_stack_size(&self) -> i32 {
        self.next_stack_offset.abs()
    }

    // Optionally: implement allocate_register if you’re doing register allocation
}
