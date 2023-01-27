use std::str::FromStr;

use proc_macro::TokenStream;




#[proc_macro]
pub fn swap_rows(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let (mut vec, mut rows) = input_str.split_once(";").expect("Error getting args");
    
    vec = vec.trim();
    rows = rows.trim();

    let (mut this, mut that) = rows.split_once(">").expect("Error with row nums");

    this = this.trim();
    that = that.trim();

   
    let fin_stream = format!(r#"
        {vec}.swap({this}, {that});
    "#);

    TokenStream::from_str(&fin_stream).expect("Error putting stream")
}

fn replace_scalar(operand: &str, rhs: &str, operator: &str) -> String {
    format!("{operand}.iter().cloned().map(|it| it {operator} {rhs}).collect::<Vec<_>>()")
}

fn replace_vector(operand: &str, rhs: &str, operator: &str) -> String {
    format!("{operand}.iter().cloned().zip({rhs}.iter().cloned()).map(|(op, rhs)| op {operator} rhs).collect::<Vec<_>>()")
}

fn replace_accumulator(operand: &str, rhs: &str, operator: &str) -> String {
    format!("{operand}.iter().cloned().zip({rhs}.iter().cloned()).map(|(op, rhs)| op {operator} rhs).sum::<_>()")
}

#[proc_macro]
pub fn vec_op(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let mut split_space = input_str.split_whitespace();

    let operand = split_space.next().expect("Needs operand");
    let operator = split_space.next().expect("Needs operator");
    let rhs = split_space.next().expect("Needs rhs");
    let sca_vec = split_space.next().expect("Needs to specify whether scalar or vec");

    let fin_str = match sca_vec.contains("sca") {
        true => format!("{{ {} }}", replace_scalar(operand, rhs, operator)),
        false => {
            match sca_vec.contains("vec") {
                true => format!("{{ {} }}", replace_vector(operand, rhs, operator)),
                false => {
                    match sca_vec.contains("acc") {
                        true => format!("{{ {} }}", replace_accumulator(operand, rhs, operator)),
                        false => panic!("Last arg must be either 'vec[tor]' or 'sca[lar]'"),
                    }
                },
            }
        },
    };

    TokenStream::from_str(&fin_str).expect("Error putting stream together")
}


#[proc_macro]
pub fn make_vec(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    
    let mut split = input_str.split('$');

    let (ty, init, num) = (
        split.next().expect("Error type"),
        split.next().expect("Error initializer"),
        split.next().expect("Error size"),

    );

    let final_str = format!(r#"
        {{ 
            let mut vec: Vec<{ty}> = vec![];  
            
            (0..{num}).into_iter().for_each(|_| {{
                let new_item = {init};
                vec.push(new_item);
            }} );

            vec
        }}"#);

    TokenStream::from_str(&final_str).expect("Error putting stream back together")
}