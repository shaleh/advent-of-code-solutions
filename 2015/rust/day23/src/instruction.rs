#[derive(Debug)]
pub enum Instruction {
    Hlf(usize),
    Tpl(usize),
    Inc(usize),
    Jmp(i64),
    Jie((usize, i64)),
    Jio((usize, i64)),
}
