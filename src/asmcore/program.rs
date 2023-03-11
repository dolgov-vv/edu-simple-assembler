
use std::collections::HashMap;
use super::*;

#[derive(Debug, Default)]
pub struct Program {
    commands: Vec<Command>,
    reg_names: Vec<String>,
    labels: HashMap<String, IP>
}

impl Program {
    pub fn get_or_create_reg(&mut self, name: &str) -> RegIndex {
        self.reg_names.iter()
            .position(|x| x == name)
            .or_else(|| {
                self.reg_names.push(name.to_string());
                Some(self.reg_names.len()-1)
            }).unwrap()
    }

    pub fn get_reg_count(&self) -> usize { self.reg_names.len() }

    pub fn append_command(&mut self, command: Command) {
        self.commands.push(command)
    }

    pub fn get_command_count(&self) -> usize { self.commands.len() }

    #[inline]
    pub fn get_command(&self, ip: IP) -> Option<&Command> {
        self.commands.get(ip)
    }

    pub fn set_labels(&mut self, labels: impl Iterator<Item = (Label, IP)>) {
        self.labels = HashMap::from_iter(labels);
    }

    pub fn set_label(&mut self, label: Label, ip: IP) {
        self.labels.insert(label, ip);
    }

    pub fn get_label_ip(&self, label: &str) -> Option<IP> {
        if let Some(&ip) = self.labels.get(label) { Some(ip) } else { None }
    }
}
