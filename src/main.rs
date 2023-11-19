mod stack;
use std::collections::HashSet;

fn main() {
    // stack::test_reverse();

    let ops : HashSet<Element> = HashSet::from([PLUS, MINUS, MUL, DIV, POW, OPEN_BRACKET, CLOSE_BRACKET]);
    let op_chars: HashSet<char> = HashSet::from(['+', '-', '*', '/', '(', ')', '^']);
    let ns = String::from("(33 + 45 / 3 *(2+9)-50)".trim());
    let mut operators: Vec<Element> = stack::new_stack(ns.len());
    let mut numbers: Vec<Element> = stack::new_stack(ns.len());

    // prev sign, could be 0
    let mut prev_char: char = ' ';
    let mut prev_op = EMPTY_OP;
    let mut num: String = String::from("");

    for c in ns.chars() {
        if c.is_numeric() {
            num = num + &*String::from(c);
            prev_char = c;
        } else if op_chars.contains(&c) { // means not empty and not
            if !num.is_empty() {
                let n: u32 = num.parse().unwrap();
                num = String::from("");
                numbers.push(Element::Number(n));
            }
            let op : Element = Element::Operation(c);
            if ops.contains(&prev_op) {
                operators.push(prev_op);
            }
            prev_op = op;

            match op {
                PLUS => prev_op = PLUS,
                MINUS => prev_op = MINUS,
                MUL => prev_op = MUL,
                DIV => prev_op = DIV,
                _ => {}
            }
        }
    }
    if !num.is_empty() {
        let n: u32 = num.parse().unwrap();
        numbers.push(Element::Number(n));
    }

    if prev_op != EMPTY_OP {
        operators.push(prev_op);
    }
    //set(&mut operators, prev_op, Element::Number(num.parse().unwrap()), prev_op);

    stack::print_stack(&operators, |op: &Element| op.to_string());
    stack::print_stack(&numbers, |op: &Element| op.to_string());
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Element {
    Operation { character: char, order: i32 },
    Number(u32)
}

impl Element {
    fn to_string(&self) -> String {
        match self {
            Element::Number(n) => n.to_string(),
            Element::Operation { character, order} => character.to_string() + "(" + &order.to_string() + ")"
        }
    }
}


const EMPTY_CHAR: char = '\0';

const PLUS : Element =  Element::Operation{character: '+', order: 0};
const MINUS : Element =  Element::Operation{character: '-', order: 0};
const MUL : Element =  Element::Operation{character: '*', order: 1};
const DIV : Element =  Element::Operation{character: '/', order: 1};
const POW : Element =  Element::Operation{character: '^', order: 2};
const OPEN_BRACKET : Element =  Element::Operation{character: '(', order: 3};
const CLOSE_BRACKET : Element =  Element::Operation{character: ')', order: 3};

const EMPTY_OP: Element =  Element::Operation{character: EMPTY_CHAR, order: -1};
