
pub fn new_stack<T>(maxsize: usize) -> Vec<T> {
    Vec::with_capacity(maxsize)
}

pub fn pop<T>(stack: &mut Vec<T>) -> Option<T> {
    if stack.is_empty() {
        None
    } else {
        stack.pop()
    }
}

pub fn push<T>(stack: &mut Vec<T>, element: T) -> () {
    if !stack.is_empty() && stack.len()-1 >= stack.capacity() {
        panic!("Exceeded capacity")
    }
    stack.push(element)
}

pub fn print_stack<T>(stack: &Vec<T>, f: fn(&T) -> String) -> () {
    print!("Stack with values: :");
    for val in stack.into_iter() {
        print!("{},", f(val))
    }

    println!();
}

pub fn test<T>(push1: T, push2: T, converter: fn(&T) -> String) {
    let mut s: Vec<T> = new_stack(10);
    push(&mut s, push1);
    print_stack(&s, converter);
    push(&mut s, push2);
    print_stack(&s, converter);
    let v = pop(&mut s);
    match v {
        None => println!("{}", "{}"),
        Some(val) => println!("Popped: {}", converter(&val))
    }
    print_stack(&s, converter);
}

pub fn read_input() -> u32 {
    let mut n: String = String::new();
    std::io::stdin().read_line(&mut n)
        .expect("failed to read input");

    let n: u32 = n.trim().parse().expect("Invalid input");
    n
}


pub fn test_reverse() {
    let mut input = String::from("Hello");
    let mut s: Vec<char> = new_stack(10);
    for char in input.chars() {
        //println!("Popped: {}", char);
        push(&mut s, char);
    }
    let size = s.len();
    for _ in 0..size {
        let v = pop(&mut s);
        match v {
            None => println!("{}", "{}"),
            Some(val) => print!("{}", val)
        }
    }
}

pub fn test_num() {
    let p1: u32 = 5;
    let p2: u32 = 3;
    test(p1,p2, |v: &u32| v.to_string());
}