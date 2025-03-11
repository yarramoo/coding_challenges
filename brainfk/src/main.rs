use std::io::{self, BufReader, Cursor, Read, Write};
use clap::Parser;
use memory::Memory;
use program::{ExecState, Program};
use std::io::BufRead;
use std::fs::File;

#[derive(Parser)]
struct Cli {
    path: Option<String>,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let reader: Box<dyn BufRead> = match cli.path {
        Some(path) => {
            let mut buf = String::new();
            let mut file = File::open(path)?;
            file.read_to_string(&mut buf)?;
            buf = buf.replace("\n", "");
            Box::new(BufReader::new(Cursor::new(buf)))
        },
        None => Box::new(BufReader::new(io::stdin())),
    };

    repl(reader)
}

fn repl<T: BufRead>(mut input: T) -> io::Result<()> {
    let mut buf = String::new();
    loop {
        if input.read_line(&mut buf)? == 0 {
            break;
        }
        print!("bfk>");
        io::stdout().flush()?;
        let mut state = State::new(buf.trim()).unwrap();
        state.run_program();
        buf.clear();
    }
    Ok(())
}


#[derive(Debug)]
enum ExecError {
    InvalidCharacter(Option<char>), 
    MemoryError(memory::MemoryError),
    ProgramError(program::ProgramError),
}

mod memory;
mod program;

#[derive(Debug)]
struct State {
    pub program: Program,
    pub memory: Memory,
}

impl State {
    fn new(input: &str) -> Result<Self, String> {
        let commands: Vec<Command> = input
            .as_bytes()
            .iter()
            .filter_map(|&c| {
                TryInto::<Command>::try_into(c).ok()
            })
            .collect::<Vec<Command>>();
        Ok(Self {
            memory: Memory::new(),
            program: Program::new(commands),
        })
    }

    fn write(&mut self, value: usize) -> Result<usize, ExecError> {
        self.memory
            .write(value)
            .map_err(|e| ExecError::MemoryError(e))
    }

    fn read(&self) -> Result<usize, ExecError> {
        self.memory
            .read()
            .map_err(|e| ExecError::MemoryError(e))
    }

    fn exec_state(&self) -> ExecState {
        self.program.exec_state()
    }

    fn get_cmd(&self) -> Result<Command, ExecError> {
        self.program
            .current_cmd()
            .map_err(|e| ExecError::ProgramError(e))
    }

    fn run_program(&mut self) -> Result<ExecState, ExecError> {
        loop {
            let exec_state = self.run_cmd()?;
            if exec_state == ExecState::Completed {
                break;
            } 
        }
        Ok(ExecState::Completed)
    }

    fn run_cmd(&mut self) -> Result<ExecState, ExecError> {
        let cmd = self.get_cmd()?;
        match cmd {
            Command::Right => self.run_right(),
            Command::Left => self.run_left(),
            Command::Inc => self.run_inc(),
            Command::Dec => self.run_dec(),
            Command::Output => self.run_output(),
            Command::Input => self.run_input(),
            Command::JumpPast => self.run_jump_past(),
            Command::JumpBack => self.run_jump_back(),
        } 
    }

    fn run_right(&mut self) -> Result<ExecState, ExecError> {
        self.memory.cursor_right()
            .map_err(|e| ExecError::MemoryError(e))?;
        self.program.increment();
        Ok(self.exec_state())
    }

    fn run_left(&mut self) -> Result<ExecState, ExecError> {
        self.memory.cursor_left()
            .map_err(|e| ExecError::MemoryError(e))?;
        self.program.increment();
        Ok(self.exec_state())
    }

    fn run_inc(&mut self) -> Result<ExecState, ExecError> {
        self.write(self.read()? + 1)?;
        self.program.increment();
        Ok(self.exec_state())
    }

    fn run_dec(&mut self) -> Result<ExecState, ExecError> {
        self.write(self.read()? - 1)?;
        self.program.increment();
        Ok(self.exec_state())
    }

    fn run_output(&mut self) -> Result<ExecState, ExecError> {
        let c = char::from(self.read()? as u8); 
        print!("{}", c);
        // let _ = io::stdout().flush();
        self.program.increment();
        Ok(self.exec_state())
    }

    fn run_input(&mut self) -> Result<ExecState, ExecError> {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf);
        let c = buf
            .chars()
            .next()
            .ok_or(ExecError::InvalidCharacter(None))?;
        if !c.is_ascii() {
            return Err(ExecError::InvalidCharacter(Some(c)));
        }
        self.write(c as u8 as usize);
        self.program.increment();
        Ok(self.exec_state())
    }

    fn run_jump_past(&mut self) -> Result<ExecState, ExecError> {
        let cell = self.memory.read()
            .map_err(|e| ExecError::MemoryError(e))?;
        if cell == 0 {
            self.program.jump_past()
                .map_err(|e| ExecError::ProgramError(e))?;
        } else {
            self.program.increment();
        }
        Ok(self.exec_state())
    }

    fn run_jump_back(&mut self) -> Result<ExecState, ExecError> {
        let cell = self.memory.read()
            .map_err(|e| ExecError::MemoryError(e))?;
        if cell != 0 {
            self.program.jump_back()
                .map_err(|e| ExecError::ProgramError(e))?;
        } else {
            self.program.increment();
        }
        Ok(self.exec_state())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Command {
    Right, Left, Inc, Dec, Output, Input, JumpPast, JumpBack,
}

impl TryFrom<u8> for Command {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'>' => Ok(Self::Right),
            b'<' => Ok(Self::Left),
            b'+' => Ok(Self::Inc),
            b'-' => Ok(Self::Dec),
            b'.' => Ok(Self::Output),
            b',' => Ok(Self::Input),
            b'[' => Ok(Self::JumpPast),
            b']' => Ok(Self::JumpBack),
            _ =>  Err(format!("Invalid input {}", value as char)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_numerical() {
        let mut state = State::new("+++++>++++>+++>++>+>").unwrap();
        state.run_program().unwrap();
        let mut predicted_memory = Box::new([0; memory::MEM_SIZE]);
        for i in 0..5 {
            predicted_memory[i] = 5 - i;
        }
        assert_eq!(state.memory.buf, predicted_memory)
    }

    #[test]
    fn test_basic_loop() {
        let mut state = State::new("+++++[-]").unwrap();
        state.run_program().unwrap();
        let predicted_memory = Box::new([0; memory::MEM_SIZE]);
        assert_eq!(state.memory.buf, predicted_memory);
    }
}