use renderer::window_renderer::*;

enum Operators {
    Add(f32, f32),     
    Subtraction(f32, f32),     
    Multiplication(f32, f32),     
    Division(f32, f32),     
    SquareRoot(f32),
}

struct History {
    lst: Vec<Operators>
}
// Option: Only to use for division by 0
fn eval_operator(op: Operators) -> Option<f32> {
   match op {
    Operators::Add(lhs, rhs) => Some(lhs + rhs),
    Operators::Subtraction(lhs, rhs) => Some(lhs - rhs),
    Operators::Multiplication(lhs, rhs) => Some(lhs * rhs),
    Operators::Division(lhs, rhs) => if rhs == 0 {None} else {Some(lhs/rhs)},
    Operators::SquareRoot(num) => Some(num.sqrt()),
}} 


fn main() {
    let texts = vec![];
    let win = renderer::window::Window::new(texts, 56, 12, TypeOfBorder::CurvedBorders);
    println!("{}", win.render().unwrap());
}

