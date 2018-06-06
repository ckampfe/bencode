#[macro_use]
extern crate combine;

pub mod bencode {
    use combine::error::ParseError;
    use combine::parser::char::{char, digit, spaces};
    use combine::stream::Stream;
    use combine::{any, between, choice, count_min_max, many, many1, optional, token, Parser};

    use std::collections::{BTreeMap, HashMap};

    pub trait Encode {
        fn encode(&self) -> String;
    }

    pub trait Decode {
        fn decode(&self) -> Bencodable;
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Bencodable {
        Dict(HashMap<String, Bencodable>),
        List(Vec<Bencodable>),
        Str(String),
        Int(isize),
    }

    fn expr_<I>() -> impl Parser<Input = I, Output = Bencodable>
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
                .map(|t| Bencodable::Int(t)),
        );

        let digit_colon = digits2
            .and(token(':'))
            .map(|res: (String, char)| res.0.parse::<usize>().unwrap());

        let string =
            digit_colon.then(|d: usize| count_min_max(d, d, any()).map(|s| Bencodable::Str(s)));

        let list = between(lex_char('l'), lex_char('e'), many(expr())).map(|l| Bencodable::List(l));

        let expr_pair = count_min_max(2, 2, expr());

        let dict =
            between(lex_char('d'), lex_char('e'), many(expr_pair)).map(|l: Vec<Vec<Bencodable>>| {
                let mut hm = HashMap::new();

                for ref pair in l.iter() {
                    let kstring = match &pair[0] {
                        Bencodable::Str(s) => s.clone(),
                        _ => panic!["ok"],
                    };

                    hm.insert(kstring, pair[1].clone());
                }

                Bencodable::Dict(hm)
            });

        choice((integer, string, list, dict)).skip(spaces())
    }

    parser!{
            fn expr[I]()(I) -> Bencodable
            where [I: Stream<Item = char>]
            {
                expr_()
            }
    }

    impl Decode for str {
        fn decode(&self) -> Bencodable {
            let result = expr().parse(self);
            match result {
                Ok((res, _)) => res,
                Err(err) => panic!(err),
            }
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

    impl Encode for usize {
        fn encode(&self) -> String {
            let mut s = String::new();
            s.push_str("i");
            s.push_str(&self.to_string());
            s.push_str("e");
            s
        }
    }

    impl Encode for u8 {
        fn encode(&self) -> String {
            let mut s = String::new();
            s.push_str("i");
            s.push_str(&self.to_string());
            s.push_str("e");
            s
        }
    }

    impl Encode for u16 {
        fn encode(&self) -> String {
            let mut s = String::new();
            s.push_str("i");
            s.push_str(&self.to_string());
            s.push_str("e");
            s
        }
    }

    impl Encode for u32 {
        fn encode(&self) -> String {
            let mut s = String::new();
            s.push_str("i");
            s.push_str(&self.to_string());
            s.push_str("e");
            s
        }
    }

    impl Encode for u64 {
        fn encode(&self) -> String {
            let mut s = String::new();
            s.push_str("i");
            s.push_str(&self.to_string());
            s.push_str("e");
            s
        }
    }

    impl Encode for u128 {
        fn encode(&self) -> String {
            let mut s = String::new();
            s.push_str("i");
            s.push_str(&self.to_string());
            s.push_str("e");
            s
        }
    }

    impl Encode for isize {
        fn encode(&self) -> String {
            let mut s = String::new();
            s.push_str("i");
            s.push_str(&self.to_string());
            s.push_str("e");
            s
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

    impl Encode for Vec<Bencodable> {
        fn encode(&self) -> String {
            let mut s: String = String::new();

            s.push_str("l");

            for item in self.iter() {
                let encoded_value = match item {
                    Bencodable::Str(s) => s.encode(),
                    Bencodable::Int(i) => i.encode(),
                    Bencodable::List(l) => l.encode(),
                    Bencodable::Dict(d) => d.encode()
                };
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

    impl Encode for HashMap<String, Bencodable> {
        fn encode(&self) -> String {
            // "Keys must be strings and appear in sorted order"
            // http://www.bittorrent.org/beps/bep_0003.html
            let btreemap: BTreeMap<_, _> = self.iter().collect();

            let mut s: String = String::new();

            s.push_str("d");

            for (key, value) in btreemap {
                let encoded_key = key.encode();
                let encoded_value = match value {
                    Bencodable::Str(s) => s.encode(),
                    Bencodable::Int(i) => i.encode(),
                    Bencodable::List(l) => l.encode(),
                    Bencodable::Dict(d) => d.encode()
                };
                s.push_str(&encoded_key);
                s.push_str(&encoded_value.to_string());
            }

            s.push_str("e");

            s
        }
    }
}

#[cfg(test)]
mod tests {
    use self::Bencodable::*;
    use bencode::{Bencodable, Decode, Encode};
    use std::collections::HashMap;
    use std::{usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128};

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
    fn encodes_usize() {
        let mut usize_max = String::new();
        usize_max.push_str("i");
        usize_max.push_str(&usize::MAX.to_string());
        usize_max.push_str("e");
        let mut usize_min = String::new();
        usize_min.push_str("i");
        usize_min.push_str(&usize::MIN.to_string());
        usize_min.push_str("e");

        assert_eq!(usize::MAX.encode(), usize_max);
        assert_eq!(usize::MIN.encode(), usize_min);
        assert_eq!(0usize.encode(), "i0e");
    }

    #[test]
    fn encodes_u8() {
        let mut u8_max = String::new();
        u8_max.push_str("i");
        u8_max.push_str(&u8::MAX.to_string());
        u8_max.push_str("e");
        let mut u8_min = String::new();
        u8_min.push_str("i");
        u8_min.push_str(&u8::MIN.to_string());
        u8_min.push_str("e");

        assert_eq!(u8::MAX.encode(), u8_max);
        assert_eq!(u8::MIN.encode(), u8_min);
        assert_eq!(0u8.encode(), "i0e");
    }

    #[test]
    fn encodes_u16() {
        let mut u16_max = String::new();
        u16_max.push_str("i");
        u16_max.push_str(&u16::MAX.to_string());
        u16_max.push_str("e");
        let mut u16_min = String::new();
        u16_min.push_str("i");
        u16_min.push_str(&u16::MIN.to_string());
        u16_min.push_str("e");

        assert_eq!(u16::MAX.encode(), u16_max);
        assert_eq!(u16::MIN.encode(), u16_min);
        assert_eq!(0u16.encode(), "i0e");
    }

    #[test]
    fn encodes_u32() {
        let mut u32_max = String::new();
        u32_max.push_str("i");
        u32_max.push_str(&u32::MAX.to_string());
        u32_max.push_str("e");
        let mut u32_min = String::new();
        u32_min.push_str("i");
        u32_min.push_str(&u32::MIN.to_string());
        u32_min.push_str("e");

        assert_eq!(u32::MAX.encode(), u32_max);
        assert_eq!(u32::MIN.encode(), u32_min);
        assert_eq!(0u32.encode(), "i0e");
    }

    #[test]
    fn encodes_u64() {
        let mut u64_max = String::new();
        u64_max.push_str("i");
        u64_max.push_str(&u64::MAX.to_string());
        u64_max.push_str("e");
        let mut u64_min = String::new();
        u64_min.push_str("i");
        u64_min.push_str(&u64::MIN.to_string());
        u64_min.push_str("e");

        assert_eq!(u64::MAX.encode(), u64_max);
        assert_eq!(u64::MIN.encode(), u64_min);
        assert_eq!(0u64.encode(), "i0e");
    }

    #[test]
    fn encodes_u128() {
        let mut u128_max = String::new();
        u128_max.push_str("i");
        u128_max.push_str(&u128::MAX.to_string());
        u128_max.push_str("e");
        let mut u128_min = String::new();
        u128_min.push_str("i");
        u128_min.push_str(&u128::MIN.to_string());
        u128_min.push_str("e");

        assert_eq!(u128::MAX.encode(), u128_max);
        assert_eq!(u128::MIN.encode(), u128_min);
        assert_eq!(0u128.encode(), "i0e");
    }

    #[test]
    fn encodes_isize() {
        let mut isize_max = String::new();
        isize_max.push_str("i");
        isize_max.push_str(&isize::MAX.to_string());
        isize_max.push_str("e");
        let mut isize_min = String::new();
        isize_min.push_str("i");
        isize_min.push_str(&isize::MIN.to_string());
        isize_min.push_str("e");

        assert_eq!(isize::MAX.encode(), isize_max);
        assert_eq!(isize::MIN.encode(), isize_min);
        assert_eq!(0isize.encode(), "i0e");
    }

    #[test]
    fn encodes_i8() {
        assert_eq!(i8::MAX.encode(), "i127e");
        assert_eq!(i8::MIN.encode(), "i-128e");
        assert_eq!(0i8.encode(), "i0e");
    }

    #[test]
    fn encodes_i16() {
        assert_eq!(i16::MAX.encode(), "i32767e");
        assert_eq!(i16::MIN.encode(), "i-32768e");
        assert_eq!(0i16.encode(), "i0e");
    }
    #[test]
    fn encodes_i32() {
        assert_eq!(i32::MAX.encode(), "i2147483647e");
        assert_eq!(i32::MIN.encode(), "i-2147483648e");
        assert_eq!(0i32.encode(), "i0e");
    }

    #[test]
    fn encodes_i64() {
        assert_eq!(i64::MAX.encode(), "i9223372036854775807e");
        assert_eq!(i64::MIN.encode(), "i-9223372036854775808e");
        assert_eq!(0i64.encode(), "i0e");
    }

    #[test]
    fn encodes_i128() {
        assert_eq!(i128::MAX.encode(), "i170141183460469231731687303715884105727e");
        assert_eq!(i128::MIN.encode(), "i-170141183460469231731687303715884105728e");
        assert_eq!(0i128.encode(), "i0e");
    }

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
        assert_eq!("i1e".decode(), Int(1));
        assert_eq!("i0e".decode(), Int(0));
        assert_eq!("i-1e".decode(), Int(-1));
        assert_eq!("i-1e".to_string().decode(), Int(-1));
    }

    #[test]
    fn decodes_strings() {
        assert_eq!("4:fcuk".decode(), Str("fcuk".to_string()));
        assert_eq!("0:".decode(), Str("".to_string()));
        assert_eq!("6:yellow".decode(), Str("yellow".to_string()));
    }

    #[test]
    fn decodes_lists() {
        assert_eq!("le".decode(), List(vec![]));
        assert_eq!("l4:fcuke".decode(), List(vec![Str("fcuk".to_string())]));
        assert_eq!(
            "l4:fcuki99ee".decode(),
            List(vec![Str("fcuk".to_string()), Int(99)])
        );
        assert_eq!("lli1024eee".decode(), List(vec![List(vec![Int(1024)])]));
    }

    #[test]
    fn decodes_dicts() {
        let mut hm = HashMap::new();
        hm.insert("hi".to_string(), Str("there".to_string()));

        assert_eq!("d2:hi5:theree".decode(), Dict(hm));

        let mut hm2 = HashMap::new();
        hm2.insert("hi".to_string(), List(vec![Int(1), Int(2), Int(3)]));

        assert_eq!("d2:hili1ei2ei3eee".decode(), Dict(hm2));

        let mut hm3 = HashMap::new();
        let mut hm4 = HashMap::new();
        hm4.insert("inner".to_string(), Int(5));
        hm3.insert("outer".to_string(), Dict(hm4));

        assert_eq!("d5:outerd5:inneri5eee".decode(), Dict(hm3));
    }
}
