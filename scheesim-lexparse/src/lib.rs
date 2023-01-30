use std::collections::HashMap;

pub enum EqualProperty {
    Bias(String),
    Type(String),
    Resistance(String),
    Conductance(String),
    Capacitance(String),
    Inductance(String),
    Voltage(String),
    Current(String),
    Power(String),
    MTETerminalPair(String, String),
}

impl EqualProperty {
    pub fn from(s: &str) -> Self {
        let (key, value) = s.split_once("=").expect(format!("Error with key-value equal property: {s} ").as_str());

        match key.trim().to_lowercase().as_str() {
            "bias" => Self::Bias(value.trim().to_string()),
            "type" => Self::Type(value.trim().to_string()),
            "resistance" => Self::Resistance(value.trim().to_string()),
            "conductance" => Self::Conductance(value.trim().to_string()),
            "capacitance" => Self::Capacitance(value.trim().to_string()),
            "inductance" => Self::Inductance(value.trim().to_string()),
            "voltage" => Self::Voltage(value.trim().to_string()),
            "current" => Self::Current(value.trim().to_string()),
            "power" => Self::Power(value.trim().to_string()),
            _ => Self::MTETerminalPair(key.trim().to_string(), value.trim().to_string()),
        }
    }

    pub fn to_unit(&self) -> Option<Unit> {
        match self {
            EqualProperty::Bias(_) => None,
            EqualProperty::Type(_) => None,
            EqualProperty::Resistance(value) => Some(Unit::from(value)),
            EqualProperty::Conductance(value) => Some(Unit::from(value)),
            EqualProperty::Capacitance(value) => Some(Unit::from(value)),
            EqualProperty::Inductance(value) => Some(Unit::from(value)),
            EqualProperty::Voltage(value) => Some(Unit::from(value)),
            EqualProperty::Current(value) => Some(Unit::from(value)),
            EqualProperty::Power(value) => Some(Unit::from(value)),
            EqualProperty::MTETerminalPair(_, _) => None,
        }
    }

    pub fn to_bias(&self) -> Option<Bias> {
        match self {
            EqualProperty::Bias(value) => Some(Bias::from(value)),
            _ => None,
        }
    }

    pub fn to_type(&self) -> Option<ElementType> {
        match self {
            EqualProperty::Type(value) => Some(ElementType::from(value)),
            _ => None,
        }
    }

    pub fn to_terminal_pair(&self) -> Option<MTETerminalPair> {
        match self {
            EqualProperty::MTETerminalPair(connector, connectee) => Some(MTETerminalPair::from(connector, connectee)),
            _ => None,
        }   
    }
}


pub enum PadToken {
    Tab(usize),
    Space(usize),
}

impl PadToken {
    pub fn new(marker: &str) -> Self {
        if marker.chars().next().unwrap() != '[' || marker.chars().last().unwrap() != ']' {
            panic!("First line of the netlist must begin with [ and end with ], containing padding token and number of it");
        }

        let removed_left_square_bracket = marker.replace("[", "");
        let removed_right_square_bracket = removed_left_square_bracket.replace("]", "");

        let (token, num_str) = removed_right_square_bracket
            .split_once(";")
            .expect("Error getting padding token and number of it");
        let num = num_str
            .trim()
            .parse::<usize>()
            .expect("Error parsing number of padding tokens");

        match token.trim().to_lowercase().as_str() {
            "space" => Self::Space(num),
            "tab" => Self::Tab(num),
            _ => panic!(
                "Unspecified padding token; must either be `tab` or `space` (case-insensetive)"
            ),
        }
    }

    pub fn split_on(&self, s: String, stage_num: usize) -> Vec<String> {
        let seperator = self.produce_seperator(stage_num);

        s.split(&seperator).map(|s| s.to_string()).collect()
    }

    fn produce_seperator(&self, stage_num: usize) -> String {
        match self {
            PadToken::Tab(n) => "\t".repeat(n * stage_num),
            PadToken::Space(n) => " ".repeat(n * stage_num),
        }
    }
}

pub enum Bias {
    NPN,
    PNP,
    Negative,
    Positive,
    NP,
    PN,
}

impl Bias {
    pub fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "npn" => Self::NPN,
            "pnp" => Self::PNP,
            "pos" | "positive" => Self::Positive,
            "neg" | "negative" => Self::Negative,
            "np" => Self::NP,
            "pn" => Self::PN,
            _ => panic!("Wrong bias type: {}", s),
        }   
    }
}


pub struct MTETerminalPair {
    connector: String,
    connectee: String,
}


impl MTETerminalPair {
    pub fn from(connector: &String, connectee: &String) -> Self {
        Self { connector: connector.clone(), connectee: connectee.clone() }
    }
}


pub enum ElementFunction {
    Resistor(Unit),
    Capacitor(Unit),
    Inductor(Unit),
    VoltageSource(Unit),
    CurrentSource(Unit),
    CurrentControlledVoltageSource(Unit, Unit),
    CurrentControlledCurrentSource(Unit, Unit),
    VoltageControlledCurrentSource(Unit, Unit),
    VoltageControlledVoltageSource(Unit, Unit),
    BJT(Unit, Bias),
    FET(Unit, Bias),
    MOSFET(Unit, Bias),
    Diode(Unit, Bias),
    OpAmp(Unit),
    IC(MTETerminalPair, Vec<Box<ElementFunction>>),
}

impl ElementFunction {
    fn get_function_and_properties(s: &str) -> (String, String) {
        let (func, prop) = s.split_once("<").expect("Error with element input");
        let prop_sans_right_angle = prop.replace(">", "");

        (func.to_string(), prop_sans_right_angle)
    }
    
    fn parse_equal_props(s: &str) -> Vec<EqualProperty> {
        s.split(",").map(|ss| EqualProperty::from(&ss.trim())).collect()
    }

    pub fn from(s: &str) -> Self {
        let (func, props) = Self::get_function_and_properties(s);
        let props_parsed = Self::parse_equal_props(&props);
    }
}

pub enum ElementType {
    Linear,
    NonLinear,
    Dynamic,
}

impl ElementType {
    pub fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "linear" => Self::Linear,
            "nonlinear" => Self::NonLinear,
            "dynamic" => Self::Dynamic,
            _ => panic!("Wrong element type given: {value}")
        }
    }
}

pub struct Element {
    terminal: Terminal,
    func: ElementFunction,
    ty: ElementType,
    termty: ElementTerminal,
}

pub enum Terminal {
    Named(String),
    External,
    Ground,
    InternalGround,
}

pub enum ElementTerminal {
    BiTerminal(Terminal, Terminal),
    TriTerminal(Terminal, Terminal, Terminal),
    MultiTerminal(HashMap<Terminal, Terminal>),
}

pub enum NodeTerminal {
    BiTerminal(Terminal, Terminal),
    TriTerminal(Terminal, Terminal),
}

pub struct Node {
    terminal: Terminal,
    termty: NodeTerminal,
    elements: Vec<Element>,
}

pub enum UnitMultiplier {
    Quetta,
    Ronna,
    Yotta,
    Zetta,
    Exa,
    Petta,
    Tera,
    Giga,
    Mega,
    Kilo,
    Hecto,
    Deka,
    Deci,
    Centi,
    Milli,
    Micro,
    Nano,
    Pico,
    Femto,
    Atto,
    Zepto,
    Yocto,
    Ronto,
    Quecto,
    NoMultiplier,
}

impl UnitMultiplier {
    pub fn from(s: &str) -> Self {
        match s {
            "Q" => Self::Quecto,
            "R" => Self::Ronna,
            "Y" => Self::Yotta,
            "Z" => Self::Zetta,
            "E" => Self::Exa,
            "P" => Self::Petta,
            "T" => Self::Tera,
            "G" => Self::Giga,
            "M" => Self::Mega,
            "k" => Self::Kilo,
            "h" => Self::Hecto,
            "da" => Self::Deka,
            "d" => Self::Deci,
            "c" => Self::Centi,
            "m" => Self::Milli,
            "u" | "μ" => Self::Micro,
            "n" => Self::Nano,
            "p" => Self::Pico,
            "f" => Self::Femto,
            "a" => Self::Atto,
            "z" => Self::Zepto,
            "y" => Self::Yocto,
            "r" => Self::Ronto,
            "q" => Self::Quecto,
            _ => Self::NoMultiplier,
        }
    }

    pub fn multiply_unit(&self, unit: f64) -> f64 {
        match self {
            UnitMultiplier::Quetta => unit * 1e30,
            UnitMultiplier::Ronna => unit * 1e27,
            UnitMultiplier::Yotta => unit * 1e24,
            UnitMultiplier::Zetta => unit * 1e21,
            UnitMultiplier::Exa => unit * 1e18,
            UnitMultiplier::Petta => unit * 1e15,
            UnitMultiplier::Tera => unit * 1e12,
            UnitMultiplier::Giga => unit * 1e9,
            UnitMultiplier::Mega => unit * 1e6,
            UnitMultiplier::Kilo => unit * 1e3,
            UnitMultiplier::Hecto => unit * 1e2,
            UnitMultiplier::Deka => unit * 1e1,
            UnitMultiplier::Deci => unit * 1e-1,
            UnitMultiplier::Centi => unit * 1e-2,
            UnitMultiplier::Milli => unit * 1e-3,
            UnitMultiplier::Micro => unit * 1e-6,
            UnitMultiplier::Nano => unit * 1e-9,
            UnitMultiplier::Pico => unit * 1e-12,
            UnitMultiplier::Femto => unit * 1e-15,
            UnitMultiplier::Atto => unit * 1e-18,
            UnitMultiplier::Zepto => unit * 1e-21,
            UnitMultiplier::Yocto => unit * 1e-24,
            UnitMultiplier::Ronto => unit * 1e-27,
            UnitMultiplier::Quecto => unit * 1e-30,
            UnitMultiplier::NoMultiplier => unit,
        }
    }
}

pub enum Unit {
    Henry(f64, UnitMultiplier),
    Farad(f64, UnitMultiplier),
    Ohm(f64, UnitMultiplier),
    Mho(f64, UnitMultiplier),
    Hertz(f64, UnitMultiplier),
    Amps(f64, UnitMultiplier),
    Volts(f64, UnitMultiplier),
    Watts(f64, UnitMultiplier),
}

impl Unit {
    fn seperate_num_from_unit(s: &str) -> (String, String, String) {
        let mut quantitiy = String::new();
        let mut multiplier = String::new();
        let mut unit = String::new();

        for ch in s.chars() {
            if ch.is_numeric() || ch == '.' {
                quantitiy.push(ch);
            } else if ch.is_alphabetic() {
                if vec![
                    'q', 'r', 'y', 'z', 'a', 'f', 'p', 'n', 'μ', 'u', 'm', 'c', 'd', 'h', 'k', 'M',
                    'G', 'T', 'P', 'T', 'E', 'Z', 'Y', 'R', 'Q',
                ]
                .into_iter()
                .any(|u| u == ch)
                {
                    match multiplier.len() == 0 {
                        true => multiplier.push(ch),
                        false => match ch == 'a' {
                            true => multiplier.push(ch),
                            false => continue,
                        },
                    }
                } else {
                    unit.push(ch);
                }
            } else {
                panic!("Illegal character in quantity, neither unit, multiplier, nor number (with or without decimal). Illegal character is: {}", ch);
            }
        }

        (quantitiy, multiplier, unit)
    }

    pub fn from(s: &str) -> Self {
        let (value_str, mult_str, unit_str) = Self::seperate_num_from_unit(&s);

        let value: f64 = value_str
            .parse()
            .expect(format!("Error parsing value: {value_str}").as_str());
        let multiplier = UnitMultiplier::from(&mult_str);

        match unit_str.as_str() {
            "H" => Self::Henry(value, multiplier),
            "F" => Self::Farad(value, multiplier),
            "Ohm" | "ohm" | "Ω" => Self::Ohm(value, multiplier),
            "Mho" | "mho" | "ʊ" => Self::Mho(value, multiplier),
            "Hz" | "hertz" | "hz" | "Hertz" => Self::Hertz(value, multiplier),
            "v" | "volts" | "Volts" => Self::Volts(value, multiplier),
            "A" | "amps" | "Amps" => Self::Amps(value, multiplier),
            "W" | "Watts" | "watts" => Self::Watts(value, multiplier),
            _ => panic!("The given unit is not SI or wrong: {}", unit_str),
        }
    }
}

pub struct ScheesimSchema {
    padder: PadToken,
    netlist: String,
    nodes: Vec<Node>,
}
