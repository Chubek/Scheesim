use std::{fs::File, io::read_to_string, path::Component, cell::{RefCell, Ref}};
use scheesim_macro::*;

#[derive(Clone)]
enum ElementMarker {
    ACSweep,
    DCSource,
    Resistor,
    Capacitor,
    Inductor,
    Transistor,
    Diode,
}

#[macro_export]
macro_rules! error_out {
    ($m:literal, $($f:ident),*) => {
        {
            let string = format!($m, $($f),*);
            eprintln!("\x1b[1;31mError:\x1b[0m {}", string);
            std::process::exit(1);
        }

    };
}

macro_rules! copy_vec {
    ($v:ident) => {
        {
            let mut vc = vec![];

            for itm in $v {
                vc.push(itm.clone());
            }

            vc
        }
    };
}

impl ElementMarker {
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

#[derive(Clone)]
enum Unit {
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

#[derive(Clone)]
enum JunctionChannel {
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

#[derive(Clone)]
enum Connection {
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

#[derive(Clone)]
enum Currentage {
    Solo(Unit),
    Dom(Unit),
    Sub(Unit),
}

#[derive(Clone)]
enum Argument {
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
                        "-parallel" => Self::Out(Connection::from(&value, false)),

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

    pub fn is_key(&self, key: &'static str) -> bool {
        match key {
            "author" => {
                match self {
                    Self::Author(_) => true,
                    _ => false,
                }
            },
            "date" => {
                match self {
                    Self::Date(_) => true,
                    _ => false,
                }
            },
            "in" => {
                match self {
                    Self::In(_) => true,
                    _ => false,
                }
            },
            "out" => {
                match self {
                    Self::Out(_) => true,
                    _ => false,
                }
            },
            "base" => {
                match self {
                    Self::Base(_) => true,
                    _ => false,
                }
            },
            "voltage" => {
                match self {
                    Self::Voltage(_) => true,
                    _ => false,
                }
            },
            "voltage" => {
                match self {
                    Self::Voltage(_) => true,
                    _ => false,
                }
            },
            "voltage" => {
                match self {
                    Self::Voltage(_) => true,
                    _ => false,
                }
            },
            "max_voltage" => {
                match self {
                    Self::MaxVoltage(_) => true,
                    _ => false,
                }
            },
            "power" => {
                match self {
                    Self::Power(_) => true,
                    _ => false,
                }
            },
            "current" => {
                match self {
                    Self::Current(_) => true,
                    _ => false,
                }
            },
            "resistance" => {
                match self {
                    Self::Resistance(_) => true,
                    _ => false,
                }
            },
            "capacitance" => {
                match self {
                    Self::Capacitance(_) => true,
                    _ => false,
                }
            },
            "inductance" => {
                match self {
                    Self::Inductance(_) => true,
                    _ => false,
                }
            },
            "frequency" => {
                match self {
                    Self::Frequency(_) => true,
                    _ => false,
                }
            },
            "junction_channel" => {
                match self {
                    Self::JunctionChannel(_) => true,
                    _ => false,
                }
            },
            "dynamic" => {
                match self {
                    Self::Dynamic => true,
                    _ => false,
                }
            },
            "nonlinear" => {
                match self {
                    Self::Nonlinear => true,
                    _ => false,
                }
            }
        }  
    }
}

trait FilterArgList {
    fn filter(&self, key: &'static str) -> Vec<Argument>;
}

impl FilterArgList for Vec<Argument> {
    fn filter(&self, key: &'static str) -> Vec<Argument> {
        self
            .clone()
            .into_iter()
            .filter(|x| {
                x.is_key(key)
            })
            .collect()
    }
}


#[derive(Clone)]
enum Lexeme {
    NetlistName(String),
    Element(ElementMarker),
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

    pub fn is_arg(&self) -> bool {
        match self {
            Self::Arg(_) => true,
            _ => false,
        }
    }

    pub fn get_arg(&self) -> Option<Argument> {
        match self {
            Self::Arg(arg) => Some(arg.clone()),
            _ => None,
        }
    }

    pub fn is_propub_fname(&self) -> bool {
        match self {
            Self::ProfileName(_) => true,
            _ => false,
        }
    }

    pub fn get_propub_fname(&self) -> Option<String> {
        match self {
            Self::ProfileName(s) => Some(s.clone()),
            _ => None,
        }
    }


    pub fn is_element(&self) -> bool {
        match self {
            Self::Element(_) => true,
            _ => false,
        }
    }

    pub fn get_element(&self) -> Option<ElementMarker> {
        match self {
            Self::Element(e) => Some(e.clone()),
            _ => None,
        }
    }


    pub fn is_nlname(&self) -> bool {
        match self {
            Self::NetlistName(_) => true,
            _ => false,
        }
    }

    pub fn get_nlname(&self) -> Option<String> {
        match self {
            Self::NetlistName(nl) => Some(nl.clone()),
            _ => None,
        }
    }

    pub fn is_ndname(&self) -> bool {
        match self {
            Self::NodeName(_) => true,
            _ => false,
        }
    }

    pub fn get_ndname(&self) -> Option<String> {
        match self {
            Self::NodeName(nd) => Some(nd.clone()),
            _ => None,
        }
    }

    pub fn is_probe(&self) -> bool {
        match self {
            Self::Pobe => true,
            _ => false,
        }
    }

    pub fn is_endmarker(&self) -> bool {
        match self {
            Self::EndMarker => true,
            _ => false,
        }
    }

    pub fn is_comment(&self) -> bool {
        match self {
            Self::Comment => true,
            _ => false,
        }
    }
}

struct LexemeLine {
    lexemes: Vec<Lexeme>,
    line_number: usize,
}

impl LexemeLine {
    pub fn from(s: &str, line_number: usize) -> Self {
        let lexemes = s.split_whitespace()
        .map(|s| Lexeme::from(s, line_number))
        .collect::<Vec<Lexeme>>();

        Self { lexemes, line_number }
    }

    pub fn count_arguments(&self) -> usize {
        self.lexemes.iter().filter(|x| x.is_arg()).count()
    }

    pub fn get_args(&self) -> Vec<Argument> {
        self.lexemes
            .clone()
            .into_iter()
            .filter(
                |x| x.is_arg())
                            .map(|x| x.clone().get_arg().unwrap()
            )
            .collect()
    }

    pub fn get_elements(&self) -> Vec<ElementMarker> {
        self.lexemes
            .clone()
            .into_iter()
            .filter(
                |x| x.is_element())
                            .map(|x| x.clone().get_element().unwrap()
            )
            .collect()
    }


    pub fn get_nlname(&self) -> Vec<String> {
        self.lexemes
            .clone()
            .into_iter()
            .filter(
                |x| x.is_nlname())
                            .map(|x| x.clone().get_nlname().unwrap()
            )
            .collect()
    }

    pub fn get_ndname(&self) -> Vec<String> {
        self.lexemes
            .clone()
            .into_iter()
            .filter(
                |x| x.is_ndname())
                            .map(|x| x.clone().get_ndname().unwrap()
            )
            .collect()
    }

    pub fn get_propub_fnames(&self) -> Vec<String> {
        self.lexemes
            .clone()
            .into_iter()
            .filter(
                |x| x.is_propub_fname())
                            .map(|x| x.clone().get_propub_fname().unwrap()
            )
            .collect()
    }

    pub fn has_valid_element(&self) -> bool {
        self.get_elements().len() == 1
    }

    pub fn has_valid_arument(&self, num: usize) -> bool {
        self.get_args().len() >= num
    }

    pub fn has_valid_nlnames(&self) -> bool {
        self.get_nlname().len() == 1
    }

    pub fn has_valid_ndnames(&self) -> bool {
        self.get_ndname().len() == 1
    }

    pub fn has_valid_propub_fnames(&self) -> bool {
        self.get_ndname().len() == 1
    }

    pub fn has_probe(&self) -> bool {
        self.lexemes
            .clone()
            .into_iter()
            .filter(
                |x| x.is_probe()
            )
            .count() > 0
    }

    pub fn has_endmarker(&self) -> bool {
        self.lexemes
            .clone()
            .into_iter()
            .filter(
                |x| x.is_endmarker()
            )
            .count() > 0
    }

    pub fn has_comment(&self) -> bool {
        self.lexemes
            .clone()
            .into_iter()
            .filter(
                |x| x.is_comment()
            )
            .count() > 0
    }
}

impl IntoIterator for LexemeLine {
    type Item = Lexeme;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.lexemes.into_iter()
    }
}

impl Clone for LexemeLine {
    fn clone(&self) -> Self {
        Self { lexemes: self.lexemes.clone(), line_number: self.line_number.clone() }
    }
}


struct Netlist {
    lines: Vec<LexemeLine>,
    current_line: usize,
}

impl Netlist {
    pub fn from(netlist: &str) -> Self {
        Self {
            lines: netlist
                .lines()
                .enumerate()
                .map(|(i, s)| LexemeLine::from(s, i + 1))
                .collect(),
            current_line: 0,
        }
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

    pub fn advance(&mut self) {
        self.current_line += 1;
    }

    pub fn get_at(&self, index: usize) -> Option<RefCell<LexemeLine>> {
        match self.lines.get(index) {
            Some(lexeme_line) => Some(RefCell::new(lexeme_line.clone())),
            None => None,
        }
    }

    pub fn get_at_refcell(this: RefCell<Self>, index: usize) -> Option<RefCell<LexemeLine>> {
        let this_ref = this.borrow();

        this_ref.get_at(index)
    }

    pub fn get_curr(&self) -> Option<RefCell<LexemeLine>> {
        match self.lines.get(self.current_line) {
            Some(lexeme_line) => Some(RefCell::new(lexeme_line.clone())),
            None => None,
        }
    }

    pub fn get_curr_and_advance(netlist: &RefCell<Self>) -> Option<RefCell<LexemeLine>> {
        let mut ref_mut_this = netlist.get_mut();
        let curr = ref_mut_this.get_curr();
        ref_mut_this.advance();

        curr
    }
}


impl Clone for Netlist {
    fn clone(&self) -> Self {
        Self { lines: self.lines.clone(), current_line: self.current_line.clone() }
    }
}

struct Resistor {
    nonlinear: bool,
    resistance: f64,
}

struct Capacitor {
    nonlinear: bool,
    dynamic: bool,
    capacitance: f64,
}

struct Inductor {
    nonlinear: bool,
    dynamic: bool,
    inductance: f64,
}

enum TransistorType {
    BJT,
    MOSFET,
}

struct Transistor {
    power: f64,
    voltage: f64,
    junction_channel: JunctionChannel,
    trantype: TransistorType,
}

struct Diode {
    power: f64,
    voltage: f64,
    junction: JunctionChannel,
}

struct ACSweep {
    freq: f64,
    max_voltage: f64,
}

enum VoltAmps {
    ParentAmps(f64),
    ChildAmps(f64),
    IndependentAmps(f64),
    ParentVolts(f64),
    ChildVolts(f64),
    IndependentVolts(f64),
}


enum DCSource {
    Voltage(VoltAmps),
    Current(VoltAmps),
    CurrentByCurrent(VoltAmps, VoltAmps),
    CurrentByVoltage(VoltAmps, VoltAmps),
    VoltageByCurrent(VoltAmps, VoltAmps),
    VoltageByVoltage(VoltAmps, VoltAmps),
}

enum EelectroCircuitComponent {
    Resistor(Resistor),
    Capacitor(Capacitor),
    Inductor(Inductor),
    Transistor(Transistor),
    Diode(Diode),
    ACSweep(ACSweep),
    DCSource(DCSource),    
    Init,
}

struct ElectroCircuitSigniture {
    author: String,
    date: String,
}

enum ConnectionType {
    Named(String),
    Probe,
    Ground,
    Next,
    Previous,
    Init,
}

struct ElectoCircuitConnection {
    serial_in: ConnectionType,
    serial_out: ConnectionType,
    serial_base: Option<ConnectionType>,
    parallel: Vec<ConnectionType>,
}

impl ElectoCircuitConnection {
    pub fn init() -> Self {
        Self { 
            serial_in: ConnectionType::Init, 
            serial_out: ConnectionType::Init,
            serial_base: None, 
            parallel: vec![],
        }
    }

    pub fn modify_serial_in(&mut self, serial: ConnectionType) {
        self.serial_in = serial;
    }

    pub fn modify_serial_out(&mut self, serial: ConnectionType) {
        self.serial_out = serial;
    }

    pub fn modify_serial_base(&mut self, serial: ConnectionType) {
        self.serial_base = Some(serial);
    }

    pub fn add_parallel(&mut self, parallel: ConnectionType) {
        self.parallel.push(parallel);
    }
}

struct ElectroCircuitNodeProfile {
    name: String,
    components: Vec<EelectroCircuitComponent>,
}

impl ElectroCircuitNodeProfile {
    pub fn init() -> Self {
        Self { name: String::new(), components: vec![] }
    }

    pub fn modify_name(&mut self, name: String) {
        self.name = name;
    } 

    pub fn init_components(&mut self, num: usize) {
        self.components = make_vec!(EelectroCircuitComponent $ EelectroCircuitComponent::Init $ num);
    }

    pub fn set_nth_components(&mut self, component: EelectroCircuitComponent, n: usize) {
        self.components[n] = component;
    }
}

struct ElectroCircuitNode {
    name: String,
    profiles: Vec<ElectroCircuitNodeProfile>,
    connections: ElectoCircuitConnection,
}

impl ElectroCircuitNode {
    pub fn init() -> Self {
        Self { 
            name: String::new(), 
            profiles: vec![], 
            connections: ElectoCircuitConnection::init()
         }
    }
}

struct ElectroCircuit {
    name: String,
    author: Option<String>,
    date: Option<String>,
    nodes: Vec<ElectroCircuitNode>,
}

impl ElectroCircuit {
    pub fn new() -> RefCell<ElectroCircuit> {
        RefCell::new(Self { name: String::new(), author: None, date: None, nodes: vec![] })
    }

    pub fn modify_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn modify_author(&mut self, author: String) {
        self.author = Some(author);
    }

    pub fn modify_date(&mut self, date: String) {
        self.date = Some(date)
    }

    pub fn set_nodes_vec(&mut self, num_nodes: usize) {

    }
}


#[derive(Clone)]
enum LexerState {
    CircuitName(RefCell<LexemeLine>, RefCell<Netlist>),
    CircuitNode(RefCell<LexemeLine>, RefCell<Netlist>),
    NodeProfile(RefCell<LexemeLine>, RefCell<Netlist>),
    InOutArgs(RefCell<LexemeLine>, RefCell<Netlist>),
    UnitArgs(RefCell<LexemeLine>, RefCell<Netlist>),
    FlagArgs(RefCell<LexemeLine>, RefCell<Netlist>),
    IdentityArgs(RefCell<LexemeLine>, RefCell<Netlist>),
    EndCircuit(RefCell<LexemeLine>, RefCell<Netlist>),
    Begin(RefCell<Netlist>),
}

impl LexerState {
    pub fn parse_begin(&self) -> Option<Self> {
        match self.is_this_type("begin") {
            true => {
                let Self::Begin(netlist) = self.clone();
                match Netlist::get_curr_and_advance(&netlist) {
                    Some(lexeme_line) => Some(Self::CircuitName(lexeme_line, netlist)),
                    None => error_out!("Error on line 1: probably empty file.",)
                }
            },
            false => None,
        }

    }

    pub fn parse_circuit_name(&self) -> Option<Self> {
        match self.is_this_type("circuit_name") {
            true => {
                let (lexeme_line, netlist) = self.get_values();

                let ll_ref = lexeme_line.get_mut();
                let nl_ref = netlist.get_mut();

                let nl_match ll_ref.has_valid_nlnames() {}
            }
            false => None,
        }
    }
}

trait LexerStateFilterAndGet {
    fn is_this_type(&self, key: &'static str) -> bool;
    fn get_values(&self) -> (&RefCell<LexemeLine>, &RefCell<Netlist>);
}

impl LexerStateFilterAndGet for LexerState {
    fn is_this_type(&self, key: &'static str) -> bool {
        match key {
            "circuit_name" => match self {
                Self::CircuitName(_, _) => true,
                _ => false,
            },
            "circuit_node" => match self {
                Self::CircuitNode(_, _) => true,
                _ => false,
            },
            "node_profile" => match self {
                Self::NodeProfile(_, _) => true,
                _ => false,
            },            
            "inout_args" => match self {
                Self::InOutArgs(_, _) => true,
                _ => false,
            },
            "unit_args" => match self {
                Self::UnitArgs(_, _) => true,
                _ => false,
            },
            "flag_args" => match self {
                Self::FlagArgs(_, _) => true,
                _ => false,
            },
            "identity_args" => match self {
                Self::IdentityArgs(_, _) => true,
                _ => false,
            },
            "end_circuit" => match self {
                Self::EndCircuit(_, _) => true,
                _ => false,
            },
            "begin" => match self {
                Self::Begin(_) => true,
                _ => false,
            }
        }
    }

    fn get_values(&self) -> (&RefCell<LexemeLine>, &RefCell<Netlist>) {
        match self {
            LexerState::CircuitName(ll, nl) => (ll, nl),
            LexerState::CircuitNode(ll, nl) => (ll, nl),
            LexerState::NodeProfile(ll, nl) => (ll, nl),
            LexerState::InOutArgs(ll, nl) => (ll, nl),
            LexerState::UnitArgs(ll, nl) => (ll, nl),
            LexerState::FlagArgs(ll, nl) => (ll, nl),
            LexerState::IdentityArgs(ll, nl) => (ll, nl),
            LexerState::EndCircuit(ll, nl) => (ll, nl),
            LexerState::Begin(_) => error_out!("State non-compliant",),
        }
    }
}