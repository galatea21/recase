use unicode_segmentation::UnicodeSegmentation;

pub fn slice_into_words(input: String) -> Vec<String> {
    pub const SYMBOLS: [&str; 6] = [" ", ".", "/", "_", "-", "\\"];

    let mut words: Vec<String> = vec![];
    let mut temp_word: Vec<&str> = vec![];

    let vec_to_lowercase = |vec: &Vec<&str>| {
        (*vec)
            .clone()
            .into_iter()
            .collect::<String>()
            .to_lowercase()
    };

    for c in input.graphemes(true) {
        // slice when a symbol is detected
        if SYMBOLS.contains(&c) {
            if !temp_word.is_empty() {
                words.push(vec_to_lowercase(&temp_word));
                temp_word.clear();
            }
            continue;
        }
        // slice when an uppercase letter is detected
        if is_uppercase(c) && !temp_word.is_empty() {
            words.push(vec_to_lowercase(&temp_word));
            temp_word.clear();
        }
        temp_word.push(c);
    }
    if !temp_word.is_empty() {
        words.push(vec_to_lowercase(&temp_word));
    }

    words
}

pub fn is_uppercase(character: &str) -> bool {
    let len = character.graphemes(true).count();
    if len != 1 {
        panic!("is_uppercase only take 1 character");
    }
    character == character.to_uppercase() && character != character.to_lowercase()
}

pub fn uppercase_first_letter(word: &str) -> String {
    let s = String::from(word);
    let mut chars = s.graphemes(true);
    match chars.next() {
        None => panic!("Passing empty words"),
        Some(c) => c.to_uppercase() + chars.as_str(),
    }
}

#[cfg(test)]
mod utils_tests {
    mod uppercase_related {
        use crate::utils::*;

        #[test]
        #[should_panic]
        fn is_uppercase_zero_char() {
            assert!(is_uppercase(""));
        }

        #[test]
        #[should_panic]
        fn is_uppercase_two_plus_chars() {
            assert!(is_uppercase("SS"));
            assert!(is_uppercase("Lmao"));
        }

        #[test]
        fn is_uppercase_one_char_ascii() {
            assert!(is_uppercase("S"));
            assert!(!is_uppercase("s"));
            assert!(!is_uppercase("i"));
            assert!(is_uppercase("I"));
            assert!(!is_uppercase("."));
            assert!(!is_uppercase("?"));
            assert!(!is_uppercase("9"));
        }

        #[test]
        fn is_uppercase_one_char_utf8() {
            assert!(is_uppercase("??"));
            assert!(!is_uppercase("??"));
            assert!(!is_uppercase("??"));
            assert!(is_uppercase("??"));
            assert!(!is_uppercase("??"));
            assert!(!is_uppercase("???"));
            assert!(!is_uppercase("??"));
        }

        #[test]
        fn uppercase_std() {
            assert_eq!("??".to_uppercase(), "SS".to_string());
        }

        #[test]
        #[should_panic]
        fn test_uppercase_first_letter() {
            assert_eq!(
                uppercase_first_letter("??enevolent"),
                "SSenevolent".to_string()
            );
            assert_eq!(uppercase_first_letter("???atsuri"), "???atsuri".to_string());
            assert_eq!(
                uppercase_first_letter("???????????????"),
                "???????????????".to_string()
            );
            assert_eq!(
                uppercase_first_letter("normalForOnce"),
                "NormalForOnce".to_string()
            );
            assert_eq!(uppercase_first_letter("?"), "?".to_string());
            uppercase_first_letter("");
        }
    }

    mod test_slice_words {
        use crate::utils::*;

        use std::vec;

        #[test]
        fn slice_words_by_symbols() {
            let input = [
                String::from("god matsuri"),
                String::from("god.matsuri?"),
                String::from("god_matsuri_ahihihi"),
                String::from("god+matsuri"),
                String::from("god   / matsuri"),
            ];

            let expected_output = [
                vec![String::from("god"), String::from("matsuri")],
                vec![String::from("god"), String::from("matsuri?")],
                vec![
                    String::from("god"),
                    String::from("matsuri"),
                    String::from("ahihihi"),
                ],
                vec![String::from("god+matsuri")],
                vec![String::from("god"), String::from("matsuri")],
            ];

            let mut output: Vec<Vec<String>> = vec![];

            for s in input {
                output.push(slice_into_words(s));
            }

            assert_eq!(output, expected_output);
        }

        #[test]
        fn slice_words_by_symbols_with_utf8() {
            let input = [
                String::from("g??d m??t??ur????"),
                String::from("kami ?????????"),
                String::from("g??d m??tsuri a h?? h?? h?? h??? h???"),
            ];

            let expected_output = [
                vec![String::from("g??d"), String::from("m??t??ur????")],
                vec![String::from("kami"), String::from("?????????")],
                vec![
                    String::from("g??d"),
                    String::from("m??tsuri"),
                    String::from("a"),
                    String::from("h??"),
                    String::from("h??"),
                    String::from("h??"),
                    String::from("h???"),
                    String::from("h???"),
                ],
            ];

            let mut output: Vec<Vec<String>> = vec![];

            for s in input {
                output.push(slice_into_words(s));
            }

            assert_eq!(output, expected_output);
        }

        #[test]
        fn slice_words_by_uppercase_with_utf8() {
            let input = [
                String::from("GodMatsuri"),
                String::from("God??tsuri?"),
                String::from("GodSatsuriAhihihi"),
                String::from("god"),
                String::from("God?"),
                String::from("??odMatsuRi??sDaB??t"),
            ];
            let expected_output = [
                vec![String::from("god"), String::from("matsuri")],
                vec![String::from("god"), String::from("??tsuri?")],
                vec![
                    String::from("god"),
                    String::from("satsuri"),
                    String::from("ahihihi"),
                ],
                vec![String::from("god")],
                vec![String::from("god?")],
                vec![
                    String::from("??od"),
                    String::from("matsu"),
                    String::from("ri"),
                    String::from("??s"),
                    String::from("da"),
                    String::from("b??t"),
                ],
            ];

            let mut output: Vec<Vec<String>> = vec![];

            for s in input {
                output.push(slice_into_words(s));
            }

            assert_eq!(output, expected_output);
        }

        #[test]
        fn slice_words_by_all_methods() {
            let input = [
                String::from("God.??ts.uri!________"),
                String::from("God ???atsuri ???????????????"),
                String::from("_??od-Matsu-Ri-??s_Da B??t  "),
            ];
            let expected_output = [
                vec![
                    String::from("god"),
                    String::from("??ts"),
                    String::from("uri!"),
                ],
                vec![
                    String::from("god"),
                    String::from("???atsuri"),
                    String::from("???????????????"),
                ],
                vec![
                    String::from("??od"),
                    String::from("matsu"),
                    String::from("ri"),
                    String::from("??s"),
                    String::from("da"),
                    String::from("b??t"),
                ],
            ];

            let mut output: Vec<Vec<String>> = vec![];

            for s in input {
                output.push(slice_into_words(s));
            }

            assert_eq!(output, expected_output);
        }
    }
}
