use crate::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    cmds: Vec<Command>,
    cursor: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProgramError {
    OutOfBoundsCursor(Program),
    BadCommand(Program),
}

#[derive(PartialEq, Eq, Debug)]
pub enum ExecState {
    Running, Completed
}

impl Program {
    pub fn new(commands: Vec<Command>) -> Self {
        Self {
            cmds: commands,
            cursor: 0,
        }
    }

    pub fn current_cmd(&self) -> Result<Command, ProgramError> {
        if !self.in_bounds() {
            return Err(ProgramError::OutOfBoundsCursor(self.clone()));
        }
        Ok(self.cmds[self.cursor])
    }

    pub fn increment(&mut self) {
        self.cursor += 1;
    }

    fn decrement(&mut self) -> Result<(), ProgramError> {
        if self.cursor == 0 {
            return Err(ProgramError::OutOfBoundsCursor(self.clone()));
        }
        self.cursor -= 1;
        Ok(())
    }

    pub fn jump_past(&mut self) -> Result<(), ProgramError> {
        if self.current_cmd()? != Command::JumpPast {
            return Err(ProgramError::BadCommand(self.clone()));
        }
        let mut jmp_fwd_count = 1;
        while jmp_fwd_count != 0 {
            self.increment();
            match self.current_cmd()? {
                Command::JumpPast => jmp_fwd_count += 1,
                Command::JumpBack => jmp_fwd_count -= 1,
                _ => {},
            }
        }
        self.increment();
        Ok(())
    }

    pub fn jump_back(&mut self) -> Result<(), ProgramError> {
        if self.current_cmd()? != Command::JumpBack {
            return Err(ProgramError::BadCommand(self.clone()));
        }
        let mut jmp_back_count = 1;
        while jmp_back_count != 0 {
            self.decrement()?;
            match self.current_cmd()? {
                Command::JumpPast => jmp_back_count -= 1,
                Command::JumpBack => jmp_back_count += 1,
                _ => {},
            }
        }
        Ok(())
    }

    pub fn exec_state(&self) -> ExecState {
        if self.cursor >= self.cmds.len() {
            ExecState::Completed
        } else {
            ExecState::Running
        }
    }

    fn in_bounds(&self) -> bool {
        self.cursor < self.cmds.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_cmd() {
        let program = Program::new(vec![Command::Inc]);
        assert_eq!(program.current_cmd(), Ok(Command::Inc));
    }

    #[test]
    fn test_increment_cursor() {
        let mut program = Program::new(vec![Command::Inc, Command::JumpPast]);
        program.increment();
        assert_eq!(program.cursor, 1);
        assert_eq!(program.current_cmd(), Ok(Command::JumpPast));
    }

    #[test]
    fn test_decrement_cursor() {
        let mut program = Program::new(vec![Command::Inc, Command::JumpPast]);
        program.increment();
        assert!(program.decrement().is_ok());
        assert_eq!(program.cursor, 0);
    }

    #[test]
    fn test_decrement_cursor_out_of_bounds() {
        let mut program = Program::new(vec![Command::Inc]);
        assert!(program.decrement().is_err());
    }

    #[test]
    fn test_jump_past() {
        let mut program = Program::new(vec![Command::JumpPast, Command::Inc, Command::JumpBack, Command::Inc]);
        assert!(program.jump_past().is_ok());
        assert_eq!(program.cursor, 3);
        assert_eq!(program.current_cmd(), Ok(Command::Inc));
    }

    #[test]
    fn test_jump_past_invalid_command() {
        let mut program = Program::new(vec![Command::Inc, Command::JumpPast, Command::JumpBack]);
        assert!(program.jump_past().is_err());
    }

    #[test]
    fn test_jump_back() {
        let mut program = Program::new(vec![Command::Inc, Command::JumpPast, Command::Inc, Command::JumpBack]);
        program.cursor = 3;
        assert!(program.jump_back().is_ok());
        assert_eq!(program.cursor, 1);
    }

    #[test]
    fn test_jump_back_invalid_command() {
        let mut program = Program::new(vec![Command::Inc, Command::JumpBack, Command::JumpPast]);
        assert!(program.jump_back().is_err());
    }
}