use std::{fs::File, io::read_to_string, process::exit};

pub enum ComponentMarker {
    ACSweep,
    DCSource,
    Resistor,
    Capacitor,
    Inductor,
    Transistor,
    Diode,
}

macro_rules! error_out {
    ($m:literal, $($f:ident),*) => {
        {
            let string = format!($m, $($f),*);
            eprintln!("\x1b[1;31mError:\x1b[0m {}", string);
            exit(1);
        }

    };
}

impl ComponentMarker {
    pub fn from(s: &str, line_num: usize) -> Self {
        match s.to_lowercase().as_str() {
            ".acsweep" => Self::ACSweep,
            ".dcsource" => Self::DCSource,
            ".resistor" => Self::Resistor,
            ".capacitor" => Self::Capacitor,
            ".inductor" => Self::Inductor,
            ".transistor" => Self::Transistor,
            ".diode" => Self::Diode,
            _ => error_out!("Unknown element: {} at line {}", s, line_num),
        }
    }
}

pub enum Unit {
    Quetta(f64),
    Ronna(f64),
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
    One(f64),
    Deci(f64),
    Centi(f64),
    Milli(f64),
    Micro(f64),
    Nano(f64),
    Pico(f64),
    Femto(f64),
    Atto(f64),
    Zepto(f64),
    Yocto(f64),
    Ronto(f64),
    Quecto(f64),
}

impl Unit {
    pub fn from(s: &str, line_number: usize) -> Self {
        let (digits, unit) = {
            let mut digits = String::new();
            let mut unit = String::new();

            for (i, ch) in s.char_indices() {
                if ch.is_numeric() || ch == '.' || ch == 'e' {
                    digits.push(ch);
                } else if vec![
                    'Q', 'R', 'R', 'Y', 'Z', 'E', 'P', 'T', 'G', 'M', 'k', 'h', 'd', 'a', 'c', 'm',
                    'u', 'n', 'p', 'f', 'z', 'y', 'r', 'q',
                ]
                .contains(&ch)
                {
                    unit.push(ch);
                } else {
                    error_out!("Error in SI unit '{}' in line {}", s, line_number);
                }
            }

            (digits, unit)
        };

        let unit_parsed = digits.parse::<f64>().unwrap();

        match unit.as_str() {
            "Q" => Self::Quetta(unit_parsed),
            "R" => Self::Ronna(unit_parsed),
            "Y" => Self::Yotta(unit_parsed),
            "Z" => Self::Zetta(unit_parsed),
            "E" => Self::Exa(unit_parsed),
            "P" => Self::Peta(unit_parsed),
            "T" => Self::Tera(unit_parsed),
            "G" => Self::Giga(unit_parsed),
            "M" => Self::Mega(unit_parsed),
            "k" => Self::Kilo(unit_parsed),
            "h" => Self::Hecto(unit_parsed),
            "d" => Self::Deci(unit_parsed),
            "a" => Self::Atto(unit_parsed),
            "da" => Self::Deca(unit_parsed),
            "c" => Self::Centi(unit_parsed),
            "m" => Self::Milli(unit_parsed),
            "u" => Self::Micro(unit_parsed),
            "n" => Self::Nano(unit_parsed),
            "p" => Self::Pico(unit_parsed),
            "f" => Self::Femto(unit_parsed),
            "z" => Self::Zepto(unit_parsed),
            "y" => Self::Yocto(unit_parsed),
            "r" => Self::Ronto(unit_parsed),
            "q" => Self::Quecto(unit_parsed),
            _ => Self::One(unit_parsed),
        }
    }

    pub fn get_corresponding_value(&self) -> f64 {
        match self {
            Unit::Quetta(u) => u * 1e30,
            Unit::Ronna(u) => u * 1e27,
            Unit::Yotta(u) => u * 1e24,
            Unit::Zetta(u) => u * 1e21,
            Unit::Exa(u) => u * 1e18,
            Unit::Peta(u) => u * 1e15,
            Unit::Tera(u) => u * 1e12,
            Unit::Giga(u) => u * 1e9,
            Unit::Mega(u) => u * 1e6,
            Unit::Kilo(u) => u * 1e3,
            Unit::Hecto(u) => u * 1e2,
            Unit::Deca(u) => u * 1e1,
            Unit::One(u) => u * 1e0,
            Unit::Deci(u) => u * 1e-1,
            Unit::Centi(u) => u * 1e-2,
            Unit::Milli(u) => u * 1e-3,
            Unit::Micro(u) => u * 1e-6,
            Unit::Nano(u) => u * 1e-9,
            Unit::Pico(u) => u * 1e-12,
            Unit::Femto(u) => u * 1e-15,
            Unit::Atto(u) => u * 1e-18,
            Unit::Zepto(u) => u * 1e-21,
            Unit::Yocto(u) => u * 1e-24,
            Unit::Ronto(u) => u * 1e-27,
            Unit::Quecto(u) => u * 1e-30,
        }
    }
}

pub enum JunctionChannel {
    NPN,
    PNP,
    NP,
    PN,
    N,
    P,
}

impl JunctionChannel {
    pub fn from(s: &str, line_number: usize) -> Self {
        match s.to_lowercase().as_str() {
            "npn" => Self::NPN,
            "pnp" => Self::PNP,
            "np" => Self::NP,
            "pn" => Self::PN,
            "n" => Self::N,
            "p" => Self::P,
            _ => error_out!("Wrong junction or channel '{}' in line {}", s, line_number),
        }
    }
}

pub enum Connection {
    Serial(String),
    Parallel(String),
    Ground,
    Next,
    Prev,
}

impl Connection {
    pub fn from(s: &str, serial: bool) -> Self {
        match s.to_lowercase().as_str() {
            "prev" | "previous" => Self::Prev,
            "next" => Self::Next,
            "grnd" | "ground" => Self::Ground,
            _ => match serial {
                true => Self::Serial(s.to_string()),
                false => Self::Parallel(s.to_string()),
            }
        }
    }
}

pub enum Currentage {
    Solo(Unit),
    Dom(Unit),
    Sub(Unit),
}

pub enum Argument {
    Author(String),
    Date(String),
    In(Connection),
    Out(Connection),
    Base(Connection),
    Voltage(Currentage),
    MaxVoltage(Unit),
    Power(Unit),
    Current(Currentage),
    Inductance(Unit),
    Capacitance(Unit),
    Resistance(Unit),
    Frequency(Unit),
    JunctionChannel(JunctionChannel),
    Dynamic,
    Nonlinear,
}

impl Argument {
    pub fn from(s: &str, line_number: usize) -> Self {
        let mut split_on_equal = s.split('=');

        let name = split_on_equal.next().unwrap().trim();

        match name.to_lowercase().as_str() {
            "-dynamic" => Self::Dynamic,
            "-nonlinear" => Self::Nonlinear,
            _ => match split_on_equal.next() {
                Some(v) => {
                    if name == "-junction" || name == "-channel" {
                        let junction_channel = JunctionChannel::from(v, line_number);
                        return Self::JunctionChannel(junction_channel);
                    }

                    let value = v.trim().to_string();

                    match name.to_lowercase().as_str() {
                        "-author" => Self::Author(value),
                        "-date" => Self::Date(value),
                        "-in" => Self::In(Connection::from(&value, true)),
                        "-base" => Self::Base(Connection::from(&value, true)),
                        "-out" => Self::Out(Connection::from(&value, true)),
                        "-in*" => Self::In(Connection::from(&value, false)),
                        "-base*" => Self::Base(Connection::from(&value, false)),
                        "-out*" => Self::Out(Connection::from(&value, false)),

                        _ => {
                            let value_unit = Unit::from(&value, line_number);

                            match name.to_lowercase().as_str() {
                                "-voltage" => Self::Voltage(Currentage::Solo(value_unit)),
                                "-current" => Self::Current(Currentage::Solo(value_unit)),
                                "-voltage*" => Self::Voltage(Currentage::Dom(value_unit)),
                                "-current*" => Self::Current(Currentage::Dom(value_unit)),
                                "-voltage^" => Self::Voltage(Currentage::Sub(value_unit)),
                                "-current^" => Self::Current(Currentage::Sub(value_unit)),
                                "-max_voltage" => Self::MaxVoltage(value_unit),
                                "-power" => Self::Power(value_unit),
                                "-inducance" => Self::Inductance(value_unit),
                                "-capacitance" => Self::Capacitance(value_unit),
                                "-resistance" => Self::Resistance(value_unit),
                                "-frequency" => Self::Frequency(value_unit),
                                _ => error_out!(
                                    "Wrong type of argument: '{}' in line {}",
                                    s,
                                    line_number
                                ),
                            }
                        }
                    }
                }
                None => error_out!(
                    "Wrong argument given: '{}' in line {}. Must have value sperated by =",
                    s,
                    line_number
                ),
            },
        }
    }
}

pub enum Lexeme {
    NetlistName(String),
    Component(ComponentMarker),
    NodeName(String),
    ProfileName(String),
    Arg(Argument),
    Pobe,
    EndMarker,
    Comment,
}

impl Lexeme {
    pub fn from(s: &str, line_number: usize) -> Self {
        let mut char_iter = s.chars();

        match char_iter.next().unwrap() {
            ';' => {
                if s.chars().filter(|ch| *ch == ';').count() > 3 {
                    error_out!(
                        "Wrong lexeme '{}' at line {}: Cannot contain more than 3 semicolons.",
                        s,
                        line_number
                    );
                }

                let res = match char_iter.next() {
                    Some(ch) => match ch {
                        ';' => {
                            let res = match char_iter.next() {
                                    Some(ch) => match ch {
                                        ';' => Self::ProfileName(s.replace(";;;", "")),
                                        _ => Self::NodeName(s.replace(";;", "")),
                                    },
                                    None => error_out!("Wrong token '{}' at line {}, semicolon marker cannot be left empty", ch, line_number),
                                };

                            res
                        }
                        _ => Self::NetlistName(s.replace(";", "")),
                    },
                    None => Self::EndMarker,
                };
                res
            }
            '-' => Self::Arg(Argument::from(s, line_number)),
            '$' => match s.to_lowercase().contains("probe") {
                true => Self::Pobe,
                false => error_out!("Lexeme '{}' in line {}: it must be $PROBE", s, line_number),
            },
            '/' => Self::Comment,
            _ => error_out!(
                "Problem with lexeme '{}' in line {}: it's not a valid lexeme for Scheesim Netlist",
                s,
                line_number
            ),
        }
    }
}

pub struct LexemeLine(Vec<Lexeme>);

impl LexemeLine {
    pub fn from(s: &str, line_number: usize) -> Self {
        Self(
            s.split_whitespace()
                .map(|s| Lexeme::from(s, line_number))
                .collect::<Vec<Lexeme>>(),
        )
    }
}

pub struct Netlist(Vec<LexemeLine>);

impl Netlist {
    pub fn from(netlist: &str) -> Self {
        Self(
            netlist
                .lines()
                .enumerate()
                .map(|(i, s)| LexemeLine::from(s, i + 1))
                .collect(),
        )
    }

    pub fn from_file(fp: &str) -> Self {
        match File::open(fp) {
            Ok(desc) => match read_to_string(desc) {
                Ok(netlist) => Self::from(&netlist),
                Err(e) => error_out!("Reading file '{}' to string: {}", fp, e),
            },
            Err(e) => error_out!("Opening file '{}' for reading: {}", fp, e),
        }
    }
}
