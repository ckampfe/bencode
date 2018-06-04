#[macro_use]
extern crate combine;

mod bencode {
    use combine::error::ParseError;
    use combine::parser::char::{char, digit, spaces};
    use combine::stream::Stream;
    use combine::{any, between, choice, count_min_max, many, many1, optional, token, Parser};

    use std::collections::{BTreeMap, HashMap};

    pub trait Encode {
        fn encode(&self) -> String;
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Decoded {
        Dict(HashMap<String, Decoded>),
        List(Vec<Decoded>),
        Str(String),
        Int(isize),
    }

    fn expr_<I>() -> impl Parser<Input = I, Output = Decoded>
    where
        I: Stream<Item = char>,
        // Necessary due to rust-lang/rust#24159
        I::Error: ParseError<I::Item, I::Range, I::Position>,
    {
        let lex_char = |c| char(c).skip(spaces());

        let digits = many1(digit());
        let digits2 = many1(digit());
        let potentially_negative_digits = optional(lex_char('-')).and(digits);

        let integer = between(
            lex_char('i'),
            lex_char('e'),
            potentially_negative_digits
                .map(|res: (Option<char>, String)| match res {
                    (Some(negative), num) => {
                        let mut n = negative.to_string();
                        n.push_str(&num);
                        n.parse::<isize>().unwrap()
                    }
                    (None, num) => num.parse::<isize>().unwrap(),
                })
                .map(|t| Decoded::Int(t)),
        );

        let digit_colon = digits2
            .and(token(':'))
            .map(|res: (String, char)| res.0.parse::<usize>().unwrap());

        let string =
            digit_colon.then(|d: usize| count_min_max(d, d, any()).map(|s| Decoded::Str(s)));

        let list = between(lex_char('l'), lex_char('e'), many(expr())).map(|l| Decoded::List(l));

        let expr_pair = count_min_max(2, 2, expr());

        let dict =
            between(lex_char('d'), lex_char('e'), many(expr_pair)).map(|l: Vec<Vec<Decoded>>| {
                let mut hm = HashMap::new();

                for ref pair in l.iter() {
                    let kstring = match &pair[0] {
                        Decoded::Str(s) => s.clone(),
                        _ => panic!["ok"],
                    };

                    hm.insert(kstring, pair[1].clone());
                }

                Decoded::Dict(hm)
            });

        choice((integer, string, list, dict)).skip(spaces())
    }

    parser!{
            fn expr[I]()(I) -> Decoded
            where [I: Stream<Item = char>]
            {
                expr_()
            }
    }

    pub fn decode(s: &str) -> Decoded {
        let result = expr().parse(s);
        match result {
            Ok((res, _)) => res,
            Err(err) => panic!(err),
        }
    }

    impl Encode for String {
        fn encode(&self) -> String {
            let length = self.len();
            let mut length_string = length.to_string();
            length_string.push_str(":");
            length_string.push_str(self);
            length_string
        }
    }

    impl Encode for i8 {
        fn encode(&self) -> String {
            let mut s = String::new();
            s.push_str("i");
            s.push_str(&self.to_string());
            s.push_str("e");
            s
        }
    }

    impl Encode for i16 {
        fn encode(&self) -> String {
            let mut s = String::new();
            s.push_str("i");
            s.push_str(&self.to_string());
            s.push_str("e");
            s
        }
    }

    impl Encode for i32 {
        fn encode(&self) -> String {
            let mut s = String::new();
            s.push_str("i");
            s.push_str(&self.to_string());
            s.push_str("e");
            s
        }
    }

    impl Encode for i64 {
        fn encode(&self) -> String {
            let mut s = String::new();
            s.push_str("i");
            s.push_str(&self.to_string());
            s.push_str("e");
            s
        }
    }

    impl Encode for i128 {
        fn encode(&self) -> String {
            let mut s = String::new();
            s.push_str("i");
            s.push_str(&self.to_string());
            s.push_str("e");
            s
        }
    }

    impl<T: Encode> Encode for Vec<T> {
        fn encode(&self) -> String {
            let mut s: String = String::new();

            s.push_str("l");

            for item in self.iter() {
                let encoded_value = &item.encode();
                s.push_str(&encoded_value);
            }

            s.push_str("e");

            s
        }
    }

    impl<T: Encode> Encode for HashMap<String, T> {
        fn encode(&self) -> String {
            // "Keys must be strings and appear in sorted order"
            // http://www.bittorrent.org/beps/bep_0003.html
            let btreemap: BTreeMap<_, _> = self.iter().collect();

            let mut s: String = String::new();

            s.push_str("d");

            for (key, value) in btreemap {
                let encoded_key = key.encode();
                let encoded_value = value.encode();
                s.push_str(&encoded_key);
                s.push_str(&encoded_value);
            }

            s.push_str("e");

            s
        }
    }
}

#[cfg(test)]
mod tests {
    use self::Decoded::*;
    use bencode::{decode, Decoded, Encode};
    use std::collections::HashMap;

    #[test]
    fn encodes_strings() {
        assert_eq!(
            "abcdefghijklmnopqrstuvwxyz".to_string().encode(),
            "26:abcdefghijklmnopqrstuvwxyz"
        );
        assert_eq!("hi".to_string().encode(), "2:hi");
        assert_eq!("".to_string().encode(), "0:")
    }

    #[test]
    fn encodes_i8() {
        let i8_max = 127;
        let i8_min = -128;
        let zero = 0;
        assert_eq!(i8_max.encode(), "i127e");
        assert_eq!(i8_min.encode(), "i-128e");
        assert_eq!(zero.encode(), "i0e");
    }

    #[test]
    fn encodes_i16() {}
    #[test]
    fn encodes_i32() {}

    #[test]
    fn encodes_i64() {
        let i64_max: i64 = 9223372036854775807;
        let i64_min: i64 = -9223372036854775808;
        let zero: i64 = 0;
        assert_eq!(i64_max.encode(), "i9223372036854775807e");
        assert_eq!(i64_min.encode(), "i-9223372036854775808e");
        assert_eq!(zero.encode(), "i0e");
    }

    #[test]
    fn encodes_i128() {}

    #[test]
    fn encodes_lists() {
        let empty: Vec<i32> = Vec::new();
        assert_eq!(vec![1i8, 2i8, 3i8].encode(), "li1ei2ei3ee");
        assert_eq!(empty.encode(), "le");
    }

    #[test]
    fn encodes_dicts() {
        let mut hm = HashMap::new();
        hm.insert("hi".to_string(), 1i8);

        assert_eq!(&hm.encode(), "d2:hii1ee");

        let mut hm_list = HashMap::new();
        hm_list.insert("hi".to_string(), vec![1i8, 32i8]);

        assert_eq!(&hm_list.encode(), "d2:hili1ei32eee");

        let mut hm_hm = HashMap::new();
        let mut nested = HashMap::new();
        nested.insert("fine".to_string(), 99i8);
        hm_hm.insert("nested!".to_string(), nested);

        assert_eq!(&hm_hm.encode(), "d7:nested!d4:finei99eee");
    }

    #[test]
    fn decodes_ints() {
        assert_eq!(decode("i1e"), Int(1));
        assert_eq!(decode("i0e"), Int(0));
        assert_eq!(decode("i-1e"), Int(-1));
    }

    #[test]
    fn decodes_strings() {
        assert_eq!(decode("4:fcuk"), Str("fcuk".to_string()));
        assert_eq!(decode("0:"), Str("".to_string()));
        assert_eq!(decode("6:yellow"), Str("yellow".to_string()));
    }

    #[test]
    fn decodes_lists() {
        assert_eq!(decode("le"), List(vec![]));
        assert_eq!(decode("l4:fcuke"), List(vec![Str("fcuk".to_string())]));
        assert_eq!(
            decode("l4:fcuki99ee"),
            List(vec![Str("fcuk".to_string()), Int(99)])
        );
        assert_eq!(decode("lli1024eee"), List(vec![List(vec![Int(1024)])]));
    }

    #[test]
    fn decodes_dicts() {
        let mut hm = HashMap::new();
        hm.insert("hi".to_string(), Str("there".to_string()));

        assert_eq!(decode("d2:hi5:theree"), Dict(hm));

        let mut hm2 = HashMap::new();
        hm2.insert("hi".to_string(), List(vec![Int(1), Int(2), Int(3)]));

        assert_eq!(decode("d2:hili1ei2ei3eee"), Dict(hm2));

        let mut hm3 = HashMap::new();
        let mut hm4 = HashMap::new();
        hm4.insert("inner".to_string(), Int(5));
        hm3.insert("outer".to_string(), Dict(hm4));

        assert_eq!(decode("d5:outerd5:inneri5eee"), Dict(hm3));
    }
}
