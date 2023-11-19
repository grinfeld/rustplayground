mod stack;
use std::collections::HashSet;

fn main() {
    // stack::test_reverse();

    let ops : HashSet<Element> = HashSet::from([PLUS, MINUS, MUL, DIV]);
    let ns = String::from("1+ 3 - 1".trim());
    let mut st: Vec<Element> = stack::new_stack(ns.len());

    // prev sign, could be 0
    let mut prev_char: char = ' ';
    let mut prev_op = FIRST;
    let mut num: String = String::from("");

    for c in ns.chars() {
        if c.is_numeric() {
            num = num + &*String::from(c);
            prev_char = c;
        } else if c != EMPTY_CHAR { // means not empty and not
            let n: u32 = num.parse().unwrap();
            num = String::from("");

            let op : Element = Element::Operation(c);
            if ops.contains(&prev_op) || prev_op == FIRST {
                set(&mut st, op, Element::Number(n), prev_op);
            }

            match op {
                PLUS => prev_op = PLUS,
                MINUS => prev_op = MINUS,
                MUL => prev_op = MUL,
                DIV => prev_op = DIV,
                _ => {}
            }
        }
    }
    set(&mut st, prev_op, Element::Number(num.parse().unwrap()), prev_op);

    stack::print_stack(&st, |op: &Element| op.to_string())
}

fn set(st: &mut Vec<Element>, current_op: Element, n: Element, mut prev_op: Element) {
    st.push(n);
    if prev_op != FIRST {
        st.push(prev_op);
    }
    prev_op = current_op
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Element {
    Operation(char),
    Number(u32)
}

impl Element {
    fn to_string(&self) -> String {
        match self {
            Element::Number(n) => n.to_string(),
            Element::Operation(c) => c.to_string()
        }
    }
}


const EMPTY_CHAR: char = ' ';

const PLUS : Element =  Element::Operation('+');
const MINUS : Element =  Element::Operation('-');
const MUL : Element =  Element::Operation('*');
const DIV : Element =  Element::Operation('/');
const FIRST : Element =  Element::Operation('_');

const OPEN_BRACKET : Element =  Element::Operation('(');
const CLOSE_BRACKET : Element =  Element::Operation(')');

const EMPTY_OP: Element =  Element::Operation(' ');
