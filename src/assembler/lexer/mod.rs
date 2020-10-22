use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub struct Node<T> {
    pub start: usize,
    pub end: usize,
    pub expr: T,
}

pub type TokenNode = Node<Token>;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Int(i32),
    Float(f32),
    Ident(String),
    String(String),
    Register(usize),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declare {
    ConstI64(TokenNode, TokenNode),
    ConstString(TokenNode, TokenNode),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Call(String, Vec<TokenNode>),
    Label(String, Box<Expression>),
}


peg::parser! {
    pub grammar assembler() for str {
        rule _()                =   quiet!{[' ' | '\t']}+
        rule __()               =   quiet!{[' ' | '\t' | '\n']}+

        rule at()               = ['@']
        rule dot()              = ['.']
        rule sharp()            = ['#']
        rule colon()            = [':']
        rule dollar()           = ['$']
        rule dec()              = ['0'..='9']
        rule sign()             = ['+' | '-']
        rule alpha()            = ['a'..='z' | 'A'..='Z']
        rule alphanum()         = ['a'..='z' | 'A'..='Z' | '0'..='9']
        rule printable()        = ['a'..='z' | 'A'..='Z' | '0'..='9' | ' ']

        pub rule int() -> i32
        = raw:$(sign()? dec()+)
        { raw.parse().unwrap() }

        pub rule uint() -> u32
        = raw:$(dec()+)
        { raw.parse().unwrap() }

        pub rule float() -> f32
        = raw:$(sign()? dec()+ "."? (dec()+)?)
        { raw.parse().unwrap() }

        pub rule ident() -> String
        = s:$(alphanum()+)
        { s.to_string() }

        pub rule string() -> String
        = "'" s:$(printable()+) "'"
        { s.to_string() }

        rule label_declare() -> String
        = s:ident() colon()
        { s }

        pub rule token() -> Node<Token> = precedence!{
            start:position!() expr:@ end:position!() { Node{ start, end, expr } }
            --
            at() s:ident()      { Token::Ident(s) }
            --
            dot() s:ident()     { Token::Ident(s) }
            --
            s:label_declare()   { Token::Ident(s) }
            --
            s:string()          { Token::String(s) }
            --
            sharp() i:int()     { Token::Int(i as i32) }
            --
            sharp() f:float()   { Token::Float(f as f32) }
            --
            dollar() r:uint()   { Token::Register(r as usize) }
        }

        pub rule code_expression() -> Node<Expression> = precedence!{
            start:position!() expr:@ end:position!()  { Node{ start, end, expr } }
            label:label_declare()? __? op:ident() _? args:token() ** _ __?
            {
                match label {
                    None => Expression::Call(op, args),
                    Some(lbl) => Expression::Label(lbl, Box::new(Expression::Call(op, args))),
                }
            }
        }
        rule data_expression() -> Node<Declare> =  precedence!{
            start:position!() expr:@  end:position!()  { Node{ start, end, expr } }
            label:token() _ ".integer" _ c:token() __
            { Declare::ConstI64(label, c) }
            --
            label:token() _ ".asciiz" _ c:token()  __
            { Declare::ConstString(label, c) }
        }

        rule data_section() -> Vec<Node<Declare>>
            = expres:data_expression() *
            { expres }

        pub rule code_section() -> Vec<Node<Expression>>
            = expres:code_expression() +
            { expres }

        pub rule parse() -> (Option<Vec<Node<Declare>>>, Vec<Node<Expression>>)
            = __? ".data" __? d:data_section()? ".code" __?  c:code_section()
            { (d, c) }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn int() {
        assert_eq!(Ok(100), assembler::int("100"));
        assert_eq!(Ok(-100), assembler::int("-100"));
        assert!(assembler::int("-100-100").is_err());
        assert!(assembler::int("-100.100").is_err());
    }

    #[test]
    fn float() {
        assert_eq!(Ok(100.), assembler::float("100."));
        assert_eq!(Ok(100.100), assembler::float("100.100"));
        assert_eq!(Ok(-100.100), assembler::float("-100.100"));
    }

    #[test]
    fn token() {
        println!("{:?}", assembler::token("@label"));
    }

    #[test]
    fn code_expression() {
        let input = ".data
label: .integer #100
label1: .integer #100
label2: .integer #100
label3: .integer #100
label4: .integer #100
label5: .integer #100
label6: .integer #100
label7: .integer #100
label8: .asciiz 'somest  ring'
.code
load $0 #4
load $1 #3
label:
mul $1 $0 $2
load $0 #1
sub $2 $0 $1
ret
";
        let (data, expressions) = assembler::parse(input).expect("err");
        for expr in &data.unwrap() {
            println!("{:?}", expr);
        }
    }
}