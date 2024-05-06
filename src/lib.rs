//! Macro Wrench

/*--- Use ---------------------------------------------------------------------------------------*/

#![allow(unused)]

use proc_macro::{TokenStream, TokenTree as TT, Punct};
use quote::quote;
// use proc_macro2::{Ident, Punct, TokenTree};
use venial::parse_item;

/*--- Impl ---------------------------------------------------------------------------------------*/

#[proc_macro]
pub fn ematch(item: TokenStream) -> TokenStream {

    let mut ret = TokenStream::new();

    let mut case = Vec::<TT>::new();

    let mut items = item.into_iter().peekable();

    let source = match items.next().unwrap() {
        o @ TT::Ident(_) => o,
        it => panic!("invalid item for match target: {it:?}"),
    };

    if let TT::Punct(p) = items.next().unwrap() {
        let p = p.as_char();
        if p != ',' {
            panic!("invalid punct {p}");
        }
    } else {
        panic!("invalid token");
    }

    let othertype = match items.next().unwrap() {
            o @ TT::Ident(_) => o,
            it => panic!("invalid item for other type: {it:?}"),
    };

    if let TT::Punct(p) = items.next().unwrap() {
        let p = p.as_char();
        if p != ',' {
            panic!("invalid punct {p}");
        }
    } else {
        panic!("invalid token");
    }

    loop {

        match items.next().unwrap() {
            ref id @ TT::Ident(ref i) => {
                case.push(id.clone());

                // break;
            }

            TT::Punct(p) => {
                match p.as_char() {
                    ':' => {},
                    ',' => {
                        let match_expr = format!(
                            "match {} {{ {}::{} => {}::{}, _=> todo!() }}",
                            source,
                            case.first().unwrap(),
                            case.last().unwrap(),
                            othertype,
                            case.last().unwrap(),
                        );
                        //panic!("{match_expr}");

                       ret.extend(match_expr.parse::<TokenStream>().unwrap())
                    },

                    _ => {
                        panic!("unhandled punct {p:?}");
                    }
                }
                if p.as_char() != ':' {
               }
            }

            unhandled => panic!("unhandled item {unhandled}"),
        }
        if items.peek().is_none() {
            break;
        }
    }

    ret
}

/*--- Test ----------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

/*--------------------------------------------- EOF ----------------------------------------------*/
