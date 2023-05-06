
fn new_stack<T>(maxsize: usize) -> Vec<T> {
    Vec::with_capacity(maxsize)
}

fn pop<T>(stack: &mut Vec<T>) -> Option<T> {
    if stack.is_empty() {
        None
    } else {
        stack.pop()
    }
}

fn push<T>(stack: &mut Vec<T>, element: T) -> () {
    if !stack.is_empty() && stack.len()-1 >= stack.capacity() {
        panic!("Exceeded capacity")
    }
    stack.push(element)
}

fn print_stack<T>(stack: &Vec<T>, f: fn(&T) -> String) -> () {
    print!("Stack with values: :");
    for val in stack.into_iter() {
        print!("{},", f(val))
    }

    println!();
}

fn convert(v: &u32) -> String {
    v.to_string()
}

pub fn main() {
    fn closure(v: &u32) -> String { v.to_string() }
    let mut s: Vec<u32> = new_stack(10);
    push(&mut s, 5);
    print_stack(&s, convert);
    push(&mut s, 3);
    print_stack(&s, convert);
    let v = pop(&mut s);
    match v {
        None => println!("{}", "{}"),
        Some(val) => println!("Popped: {}", val)
    }
    print_stack(&s, convert);
}