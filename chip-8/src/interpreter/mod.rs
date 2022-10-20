use chip8_base::Pixel;
use chip8_base::Display;
use chip8_base::Interpreter;
use chip8_base::Keys;

use core::time::Duration;
use log::{info, warn};

pub struct ChipState {
    memory: [u8; 4096],
    registers: [u8; 16],
    display: Display,
    speed: Duration,
    program_counter: u16,
}

impl ChipState {

    pub fn new(speed: Duration) -> ChipState {
        ChipState {
            memory: [0; 4096],
            registers: [0; 16],
            display: [[Pixel::Black; 64]; 32],
            speed: speed,
            program_counter: 0,
        }
    }

    fn fetch(&mut self) -> (u8, u8) {
        // Reading the two bytes from memory
        let first_byte = self.memory[usize::from(self.program_counter)];
        let second_byte = self.memory[usize::from(self.program_counter + 1)];

        // Returning the two bytes of the instruction
        (first_byte, second_byte)
    }

    fn decode(&self, bytes: (u8, u8)) {
        let nibbles = ChipState::read_nibbles_from_bytes(bytes);

        
    }

    fn read_nibbles_from_bytes(bytes: (u8, u8)) -> (u8, u8, u8, u8) {
        ((bytes.0 >> 4) & 0xF, bytes.0 & 0xF, (bytes.1 >> 4) & 0xF, bytes.1 & 0xF)
    }

}

impl Interpreter for ChipState {

    fn step(&mut self, keys: &Keys) -> Option<Display> {
        // Fetrching the instructions
        let opcode = self.fetch(); 

        // Incrementing the program counter
        self.program_counter += 2;
        
        // Wrapping the program counter around
        self.program_counter %= 4096;

        // Decoding the instructions
        self.decode(opcode);

        None
    }

    fn speed(&self) -> Duration {
        self.speed
    }

    fn buzzer_active(&self) -> bool {
        false
    }

}