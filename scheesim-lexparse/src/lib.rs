pub enum Token {
    Semicolon,
    Dash,
    Equal,
    Period,
    Dollar,
    Asterisk, 
    Newline,                   
    Character(char),
}


pub struct TokenGarble {
    tokens: Vec<Token>,
    cursor: usize,
}

pub enum ResultAdvance {
    Advanced,
    EndReached,
}

impl TokenGarble {
    pub fn advance(&mut self) -> Result<ResultAdvance, ()> {
        self.cursor += 1;
        
        match self.tokens.len() == self.cursor {
            true => Ok(ResultAdvance::Advanced),
            false => Ok(ResultAdvance::EndReached),
        }
    }

    pub fn get_at_cursor(&self) -> Token {
        *self.tokens.get(self.cursor).unwrap()
    }

    pub fn get_at_cursor_and_advance(&mut self)

    pub fn peek(&self, offset: usize) -> Result<Token, ()> {
        match self.tokens.get(self.cursor + offset) {
            Some(t) => Ok(*t),
            None => Err(()),
        }
    }

 
}

pub struct Argument {
    flag: String,
    value: Option<String>,
    asterisk: bool,
}

impl Argument {
    fn from(tokens: &mut TokenGarble) -> Self {
        match tokens.get_at_cursor()
    }
}

pub enum Lexeme {
    Name(String),
    Arg(Argument),
    ProfileMarker(String),
    RawElement(String),
    ProbeMarker,
}

pub enum Unit {
    Quettam(f64),
    Robba(f64),
    Yotta(f64),
    Zetta(f64),
    Exa(f64),
    Peta(f64),
    Tera(f64),
    Giga(f64),
    Mega(f64),
    Kilo(f64),
    Hecto(f64),
    Deca(f64),
    None(f64),
    Deci(f64),
    Centi(f64),
    Milli(f64),
    Micro(f64),
    Femto(f64),
    Atto(f64),
    Zepto(f64),
    Yocto(f64),
    Ronto(f64),
    Quecto(f64),
}

pub enum DCType {
    Current,
    Voltage,
    CurrentControlsCurrent,
    CurrentControlsVoltage,
    VoltageControlsCurrent,
    VoltageControlsVoltage,
}

pub enum Bias {
    NPN,
    PNP,
    NP,
    PN,
    N,
    P,
}



pub enum Component {
    ACSweep(f64, f64),
    DCSource(Option<f64>, Option<f64>, DCType),
    Resistor(f64, bool),
    Capacitor(f64, bool, bool),
    Inductor(f64, bool, bool),
    Transistor(f64, Bias, bool),
    Diode(f64, Bias),
}

