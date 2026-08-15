use chumsky::prelude::*;

#[derive(Debug)]
pub enum Expr<'src> {
    Num(f32),
    Neg(Box<Expr<'src>>),
    Add(Box<Expr<'src>>, Box<Expr<'src>>),
    Sub(Box<Expr<'src>>, Box<Expr<'src>>),
    Mul(Box<Expr<'src>>, Box<Expr<'src>>),
    Div(Box<Expr<'src>>, Box<Expr<'src>>),
    Var(&'src str),
    Pow(Box<Expr<'src>>, Box<Expr<'src>>),
    Call {
        func: &'src str,
        arg: Box<Expr<'src>>,
    },
}

pub fn parser<'src>() -> impl Parser<'src, &'src str, Expr<'src>> {
    let ident = text::ascii::ident().padded();
    let expr = recursive(|expr| {
        let num = text::int(10)
            .then(just('.').then(text::digits(10)).or_not())
            .to_slice()
            .map(|s: &str| Expr::Num(s.parse().unwrap()));

        let var = ident.map(Expr::Var);
        let call = ident
            .then_ignore(just('('))
            .then(expr.clone())
            .then_ignore(just(')'))
            .map(|(func, arg)| Expr::Call {
                func,
                arg: Box::new(arg),
            });
        let atom = call
            .or(num)
            .or(var)
            .or(expr.clone().delimited_by(just('('), just(')')))
            .padded();
        let op = |c| just(c).padded();

        let pow = atom.clone().foldl(
            op("^").to(Expr::Pow as fn(_, _) -> _).then(atom).repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );

        let unary = op("-")
            .repeated()
            .foldr(pow.clone(), |_op, rhs| Expr::Neg(Box::new(rhs)));

        let product = unary.clone().foldl(
            choice((
                op("*").to(Expr::Mul as fn(_, _) -> _),
                op("/").to(Expr::Div as fn(_, _) -> _),
            ))
            .then(unary)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );

        let sum = product.clone().foldl(
            choice((
                op("+").to(Expr::Add as fn(_, _) -> _),
                op("-").to(Expr::Sub as fn(_, _) -> _),
            ))
            .then(product)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );
        sum
    });
    expr
}

pub fn eval<'src>(expr: &'src Expr<'src>, x: f32, y: f32) -> f32 {
    match expr {
        Expr::Num(x) => *x,
        Expr::Neg(a) => -eval(a, x, y),
        Expr::Add(a, b) => eval(a, x, y) + eval(b, x, y),
        Expr::Sub(a, b) => eval(a, x, y) - eval(b, x, y),
        Expr::Mul(a, b) => eval(a, x, y) * eval(b, x, y),
        Expr::Div(a, b) => eval(a, x, y) / eval(b, x, y),
        Expr::Var("x") => x,
        Expr::Var("y") => y,
        Expr::Pow(a, b) => eval(a, x, y).powf(eval(b, x, y)),
        Expr::Call { func, arg } => {
            let v = eval(arg, x, y);
            match *func {
                "sin" => v.sin(),
                "cos" => v.cos(),
                "ln" => v.ln(),
                _ => todo!(),
            }
        }
        _ => todo!(),
    }
}
