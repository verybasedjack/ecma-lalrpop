use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub ecma262);

#[derive(Debug)]
pub enum Expr {}

pub fn parse<'a, T>(source: T) -> ()
where
    T: AsRef<&'a str>,
{
    let _ = ecma262::NumberParser::new().parse(source.as_ref());
}
