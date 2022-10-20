use core::time::Duration;

mod interpreter;

fn main() {

    env_logger::init();

    let interpreter = interpreter::ChipState::new(
        Duration::new(1 / 7000, 0)
    );

    chip8_base::run(interpreter);

}
