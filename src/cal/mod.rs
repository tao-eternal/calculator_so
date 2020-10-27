mod stack;

use stack::Stack;

#[derive(Debug, Default)]
pub struct Expr {
    expr: Option<Stack<String>>,
}

pub trait ExprReader<T> {
    fn read(expr: T) -> Expr;
}

impl ExprReader<&str> for Expr {
    fn read(expr: &str) -> Expr {
        match read_str(expr) {
            Some(x) => Expr { expr: Some(x) },
            None => Expr { expr: None },
        }
    }
}

impl ExprReader<String> for Expr {
    fn read(expr: String) -> Expr {
        Self::read(&*expr)
    }
}

impl Expr {
    pub fn eval(&mut self) -> String {
        let mut result = Stack::<String>::new();
        let mut rpn_expr = self.expr.as_mut().unwrap().reverse();
        while let Some(op) = rpn_expr.pop() {
            if op == "输入有误" {
                result.clear();
                break;
            }
            if is_optr(op.chars().last().unwrap_or_default()) {
                let y = result.pop().unwrap();
                let x = result.pop().unwrap();
                result.push(solve(op, x, y));
            } else {
                result.push(op);
            }
        }
        result.pop().unwrap_or_else(|| "输入有误".to_string())
    }
}

fn solve(op: String, x: String, y: String) -> String {
    let mut result = String::new();
    match &*op {
        "^" => {
            let x: f64 = x.parse().unwrap_or_default();
            let y: f64 = y.parse().unwrap_or_default();
            result.push_str((x.powf(y)).to_string().as_ref());
        }
        "x" | "*" => {
            let x: f64 = x.parse().unwrap_or_default();
            let y: f64 = y.parse().unwrap_or_default();
            result.push_str((x * y).to_string().as_ref())
        }
        "/" | "÷" => {
            let x: f64 = x.parse().unwrap_or_default();
            let y: f64 = y.parse().unwrap_or_default();
            result.push_str((x / y).to_string().as_ref())
        }
        "%" => {
            let x: f64 = x.parse().unwrap_or_default();
            let y: f64 = y.parse().unwrap_or_default();
            result.push_str((x % y).to_string().as_ref())
        }
        "+" => {
            let x: f64 = x.parse().unwrap_or_default();
            let y: f64 = y.parse().unwrap_or_default();
            result.push_str((x + y).to_string().as_ref())
        }
        "-" => {
            let x: f64 = x.parse().unwrap_or_default();
            let y: f64 = y.parse().unwrap_or_default();
            result.push_str((x - y).to_string().as_ref())
        }
        _ => {}
    }
    result
}
fn pre_deal(expr: &str) -> String {
    let mut s = String::new();
    for i in expr.chars() {
        match i {
            '0'..='9' | '.' | '+' | '-' | 'x' | '*' | '/' | '÷' | '%' | '^' | '(' | ')' => {
                s.push(i);
            }
            _ => {}
        }
    }
    s.push('\n');
    s
}
fn read_str(expr: &str) -> Option<Stack<String>> {
    //后缀表达式
    let mut rpn_expr = Stack::<String>::new();
    //临时操作符栈
    let mut optr = Stack::<String>::new();
    //记录临时操作符栈中的左括号个数
    let mut has_left = 0;
    //临时数字栈
    let mut number = String::new();
    //数字是否合法
    let mut number_dots = 0;
    //该表达式是否合法
    let mut is_invalid = false;
    //预处理去除不相干字母
    let expr = &*pre_deal(expr);
    //遍历中缀表达式
    for i in expr.chars() {
        //向后缀表达式中压入数字
        if !of_num(i) && !number.is_empty() {
            if number_dots > 1 {
                is_invalid = true;
                break;
            }
            rpn_expr.push(number.clone());
            number.clear();
            number_dots = 0;
        }
        //操作符进行操作
        if i == '(' {
            optr.push("(".to_string());
            has_left += 1;
        } else if i == ')' {
            if has_left > 0 {
                'l: while let Some(x) = optr.pop() {
                    if &*x == "(" {
                        has_left -= 1;
                        break 'l;
                    }
                    rpn_expr.push(x)
                }
            } else {
                is_invalid = true;
                break;
            }
        } else if is_optr(i) {
            //当前遍历到的操作符的优先级
            let p = get_priority(i);
            'r: loop {
                //当前操作符优先级大就先入操作符栈，否则从操作符栈取出到后缀表达式中知道当前操作符优先级大
                if p > get_priority(optr.top().unwrap_or_default().pop().unwrap_or_default()) {
                    optr.push(i.to_string());
                    break 'r;
                }
                let x = optr.pop().unwrap();
                if &*x != "(" {
                    rpn_expr.push(x);
                }
            }
        } else if i == '\n' {
            //取出剩余的操作符
            while let Some(x) = optr.pop() {
                rpn_expr.push(x)
            }
        } else {
            if i == '.' {
                number_dots += 1;
            }
            //压入每一位数字，包括小数点
            number.push(i);
        }
    }
    if is_invalid || rpn_expr.size() & 1 == 0 {
        rpn_expr.clear();
        rpn_expr.push("输入有误".to_string());
    }
    Some(rpn_expr)
}

fn get_priority(optr: char) -> isize {
    match optr {
        '+' | '-' => 1,
        'x' | '*' | '/' | '÷' | '%' => 2,
        '^' => 3,
        _ => 0,
    }
}

fn is_optr(c: char) -> bool {
    get_priority(c) != 0
}

fn of_num(c: char) -> bool {
    match c {
        '0'..='9' | '.' => true,
        _ => false,
    }
}
