const ABBREVIATION_MAP_JSON: &str = include_str!("abbreviation_map.json");

pub mod processor {
    use super::*;
    // use std::fs;
    use serde_json::Value;
    use std::collections::HashMap;
    use fancy_regex::Regex;
    use regex::Regex as SecondRegex;

    struct PunctuationConfig {
        period: String,
        question_mark: String,
        exclamation_mark: String,
        other_punctuations: Vec<String>,
    }

    struct LanguageConfig {
        alphabet_regex: Regex,
        three_consecutive_rule: Regex,
        four_consecutive_rule: Regex,
        four_space_rule: Regex,
        between_single_quotes_regex: Regex,
        between_single_quote_slanted_regex: Regex,
        roman_numerals_regex: Regex,
        question_mark_in_quotation_regex: Regex,
        punctuation_config: PunctuationConfig,
    }

    fn process(text: &str, config: LanguageConfig) -> Vec<String> {
        // step 1 : eliminate non-alphabet
        let mut filtered_string = config.alphabet_regex.replace_all(&text, "").to_string();

        // step 2 : remove numbered list (ex 1., 2., ...)
        let numbered_list_regex = Regex::new(r"\d+\.\s*").unwrap();
        numbered_list_regex.replace_all(&filtered_string, " ").to_string();

        // step 3 : mask abbreviations
        // let abb_data = fs::read_to_string("src/abbreviation_map.json").expect("Unable to read file");
        // let json: Value = serde_json::from_str(&abb_data).expect("Unable to parse JSON");
        // let vec_of_vecs: Vec<Vec<String>> = serde_json::from_value(json).expect("Unable to convert to Vec<Vec<String>>");
        // let mut abbreviations: HashMap<&str, &str> = HashMap::new();
        // for pair in vec_of_vecs {
        //     abbreviations.insert(Box::leak(pair[0].clone().into_boxed_str()), Box::leak(pair[1].clone().into_boxed_str()));
        // }

        // for (key, value) in abbreviations {
        //     let escaped_key = key.replace(".", r"\.");
        //     let key_regex = Regex::new(&format!(r"(?<!\S){}(?!\w)", escaped_key)).unwrap();
        //     filtered_string = key_regex.replace_all(&filtered_string, value).to_string();
        // }

        let json: Value = serde_json::from_str(ABBREVIATION_MAP_JSON).expect("Unable to parse JSON");
        let vec_of_vecs: Vec<Vec<String>> = serde_json::from_value(json).expect("Unable to convert to Vec<Vec<String>>");
        let mut abbreviations: HashMap<&str, &str> = HashMap::new();
        for pair in vec_of_vecs {
            abbreviations.insert(Box::leak(pair[0].clone().into_boxed_str()), Box::leak(pair[1].clone().into_boxed_str()));
        }

        // step 4 : number rules
        let period_before_number_rule = Regex::new(r"\.(?=\d)").unwrap();
        let number_after_period_before_letter_rule = Regex::new(r"(?<=\d)\.(?=\S)").unwrap();
        let newline_number_period_space_letter_rule = Regex::new(r"(?<=\r\d)\.(?=(\s\S)|\))").unwrap();
        let start_line_number_period_rule = Regex::new(r"(?<=^\d)\.(?=(\s\S)|\))").unwrap();
        let start_line_two_digit_number_period_rule = Regex::new(r"(?<=^\d\d)\.(?=(\s\S)|\))").unwrap();

        filtered_string = period_before_number_rule.replace_all(&filtered_string, " ").to_string();
        filtered_string = number_after_period_before_letter_rule.replace_all(&filtered_string, " ").to_string();
        filtered_string = newline_number_period_space_letter_rule.replace_all(&filtered_string, " ").to_string();
        filtered_string = start_line_number_period_rule.replace_all(&filtered_string, " ").to_string();
        filtered_string = start_line_two_digit_number_period_rule.replace_all(&filtered_string, " ").to_string();

        // step 5 : remove continuous punctuation
        let continuous_punctuation_regex = Regex::new(r"([؟!?\.。！:؛~\u{0964}]{2,})(\s|\z)").unwrap();
        filtered_string = continuous_punctuation_regex.replace_all(&filtered_string, " ").to_string();

        // step 6 : remove numbered references
        let numbered_reference_regex = Regex::new(r"([^\d\s])(\u{002E}|∯)((\[(\d{1?,3},?\s?-?\s?)?\b\d{1,3}\])+|((\d{1,3}\s?){0,3}\d{1,3}))( )([\u{0600}-\u{06FF}])").unwrap();
        filtered_string = numbered_reference_regex.replace_all(&filtered_string, " ").to_string();

        // step 7 : mask the website domain
        let domain_regex = SecondRegex::new(r"\b(?:www\.)?([a-zA-Z0-9-]+\.[a-zA-Z]{2,})(?:\.[a-zA-Z]{2,})?\b").unwrap();
        let mut masked_string = domain_regex.replace_all(&filtered_string, |caps: &regex::Captures| {
            let domain = caps.get(0).unwrap().as_str();
            domain.replace(".", "&^&")
        }).to_string();

        // step 8 : remove email, geo-location, and file format
        let email_regex = Regex::new(r"(\w+)(\u{0040})(\w+)(\u{002E})(\w+)").unwrap();
        let geo_location_rule = Regex::new(r"([a-zA-Z]°)\u{002E}(\s*\d+)").unwrap();
        let file_format_rule = Regex::new(r"(\s)\u{002E}((jpe?g|png|gif|tiff?|pdf|ps|docx?|xlsx?|svg|bmp|tga|exif|odt|html?|txt|rtf|bat|sxw|xml|zip|exe|msi|blend|wmv|mp[34]|pptx?|flac|rb|cpp|cs|js)\s)").unwrap();
        masked_string = email_regex.replace_all(&masked_string, " ").to_string();
        masked_string = geo_location_rule.replace_all(&masked_string, " ").to_string();
        masked_string = file_format_rule.replace_all(&masked_string, " ").to_string();

        // step 9 : remove continuous extra periods
        let single_new_line_rule = Regex::new(r"\n").unwrap();
        let three_space_rule = Regex::new(r"(\s\.){3}\s").unwrap();
        let other_three_period_rule = Regex::new(r"\.\.\.").unwrap();
        let three_space_rule_japanese = Regex::new(r"(\s。){3}\s").unwrap();
        let other_three_period_rule_japanese = Regex::new(r"。。。").unwrap();
        masked_string = single_new_line_rule.replace_all(&masked_string, " ").to_string();
        masked_string = three_space_rule.replace_all(&masked_string, " ").to_string();
        masked_string = other_three_period_rule.replace_all(&masked_string, " ").to_string();
        masked_string = three_space_rule_japanese.replace_all(&masked_string, " ").to_string();
        masked_string = other_three_period_rule_japanese.replace_all(&masked_string, " ").to_string();

        // step 10 : remove quotations
        let between_double_quotes_regex = Regex::new(r#""(?>[^"\\]+|\\{2}|\\.)*""#).unwrap();
        let between_quote_arrow_regex = Regex::new(r"«(?>[^»\\]+|\\{2}|\\.)*»").unwrap();
        let between_quote_slanted_regex = Regex::new(r"“(?>[^”\\]+|\\{2}|\\.)*”").unwrap();
        let between_square_brackets_regex = Regex::new(r"\[(?>[^\]\\]+|\\{2}|\\.)*\]").unwrap();
        let between_parens_regex = Regex::new(r"\((?>[^\(\)\\]+|\\{2}|\\.)*\)").unwrap();
        let word_with_leading_apostrophe = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-Z\u0621-\u064A])*'\S").unwrap();
        let between_em_dashes_regex = Regex::new(r"\-\-(?>[^\-\-])*\-\-").unwrap();
        masked_string = between_double_quotes_regex.replace_all(&masked_string, " ").to_string();        
        masked_string = between_quote_arrow_regex.replace_all(&masked_string, " ").to_string();        
        masked_string = between_quote_slanted_regex.replace_all(&masked_string, " ").to_string();        
        masked_string = between_square_brackets_regex.replace_all(&masked_string, " ").to_string();        
        masked_string = between_parens_regex.replace_all(&masked_string, " ").to_string();
        masked_string = word_with_leading_apostrophe.replace_all(&masked_string, " ").to_string();
        masked_string = between_em_dashes_regex.replace_all(&masked_string, " ").to_string();

        // step 11 : remove miscellaneous
        masked_string = config.three_consecutive_rule.replace_all(&masked_string, " ").to_string();
        masked_string = config.four_consecutive_rule.replace_all(&masked_string, " ").to_string();
        masked_string = config.four_space_rule.replace_all(&masked_string, " ").to_string();
        masked_string = config.between_single_quotes_regex.replace_all(&masked_string, " ").to_string();
        masked_string = config.between_single_quote_slanted_regex.replace_all(&masked_string, " ").to_string();
        masked_string = config.roman_numerals_regex.replace_all(&masked_string, " ").to_string();

        // step 12 : remove extra white space
        let extra_white_space_rule = Regex::new(r"\s{1,}").unwrap();
        masked_string = extra_white_space_rule.replace_all(&masked_string, " ").to_string();

        // step 13 : mask exclamation words
        let exclamation_word_masking_map: HashMap<&str, &str> = vec![
            ("!Xũ", "&ᓴ&Xũ"), ("!Kung", "&ᓴ&Kung"), ("ǃʼOǃKung", "&ᓴ&ʼO&ᓴ&Kung"), ("!Xuun", "&ᓴ&Xuun"), ("!Kung-Ekoka", "&ᓴ&Kung&ᓴ&Ekoka"), ("ǃHu", "&ᓴ&Hu"), ("ǃKhung", "&ᓴ&Khung"), ("ǃKu", "&ᓴ&Ku"), ("ǃung", "&ᓴ&ung"), ("ǃXo", "&ᓴ&Xo"), ("ǃXû", "&ᓴ&Xû"), ("ǃXung", "&ᓴ&Xung"), ("ǃXũ", "&ᓴ&Xũ"), ("!Xun", "&ᓴ&Xun"), ("Yahoo!", "Yahoo&ᓴ&"), ("Y!J", "Y&ᓴ&J"), ("Yum!", "Yum&ᓴ&"),
        ].into_iter().collect();
        for (key, value) in exclamation_word_masking_map {
            let key_regex = Regex::new(&format!(r"\b{}\b", key)).unwrap();
            masked_string = key_regex.replace_all(&masked_string, value).to_string();
        }

        // step 14 : apply non boundary exclamation mark rules
        let exclamation_mark_before_comma_mid_sentence_regex = Regex::new(r"(!|\u{FF01})(?=\,\s[ƁɓƊɗƘƙ\u0041-\u005A\u0061-\u007A\u00C0-\u{017F}\u{0400}-\u{04FF}\u{0600}-\u{06FF}\u{0530}-\u{058F}\u{0900}-\u{097F}\u{4E00}-\u{9FFF}\u{AC00}-\u{D7AF}\u{10A0}-\u{10FF}\u{0370}-\u{03FF}\u{0590}-\u{05FF}\u{3040}-\u{309F}\u{30A0}-\u{30FF}])").unwrap();
        masked_string = exclamation_mark_before_comma_mid_sentence_regex.replace_all(&masked_string, "&ᓴ&").to_string();

        // step 15 : mask question mark in quotation
        masked_string = config.question_mark_in_quotation_regex.replace_all(&masked_string, "&ᓷ&").to_string();

        // step 16 : sentence segmentation and unmask
        let question_mark = config.punctuation_config.question_mark.clone();
        let exclamation_mark = config.punctuation_config.exclamation_mark.clone();
        let period = config.punctuation_config.period.clone();
        let mut sentence_end_punctuation = vec![
            period.as_str(), question_mark.as_str(), exclamation_mark.as_str(),
        ];
        for punctuation in &config.punctuation_config.other_punctuations {
            sentence_end_punctuation.push(punctuation.as_str());
        }

        let mut segmented_sentence_candidates: Vec<String> = vec![];
        let mut sentence = String::new();

        for ch in masked_string.chars() {
            if sentence_end_punctuation.contains(&ch.to_string().as_str()) {
                let mut full_sentence_candidate = sentence.trim().to_string();
                full_sentence_candidate.push(ch);
                full_sentence_candidate = full_sentence_candidate.replace("&ᓷ&", config.punctuation_config.question_mark.clone().as_str());
                full_sentence_candidate = full_sentence_candidate.replace("&ᓴ&", config.punctuation_config.exclamation_mark.clone().as_str());
                full_sentence_candidate = full_sentence_candidate.replace("&^&", ".");
                full_sentence_candidate = full_sentence_candidate.trim().to_string();
                if full_sentence_candidate.len() > 2 {
                    segmented_sentence_candidates.push(full_sentence_candidate);
                    sentence = String::new();
                } else {
                    sentence = String::new();
                }
            } else {
                sentence.push(ch);
            }
        }

        if !sentence.is_empty() {
            let mut full_sentence_candidate = sentence.trim().to_string();
            full_sentence_candidate.push(config.punctuation_config.period.clone().as_str().chars().next().unwrap());
            full_sentence_candidate = full_sentence_candidate.replace("&ᓷ&", config.punctuation_config.question_mark.clone().as_str());
            full_sentence_candidate = full_sentence_candidate.replace("&ᓴ&", config.punctuation_config.exclamation_mark.clone().as_str());
            full_sentence_candidate = full_sentence_candidate.replace("&^&", ".");
            full_sentence_candidate = full_sentence_candidate.trim().to_string();
            if full_sentence_candidate.len() > 2 {
                segmented_sentence_candidates.push(full_sentence_candidate);
            }
        }

        // if the first letter is lowercase, merge it with the previous sentence
        let mut final_segmented_sentences: Vec<String> = vec![];
        let mut previous_sentence = String::new();
        for sentence_candidate in segmented_sentence_candidates {
            let first_char = sentence_candidate.chars().next().unwrap();
            if first_char.is_lowercase() {
                previous_sentence.push_str(" ");
                previous_sentence.push_str(&sentence_candidate);
            } else {
                if previous_sentence.len() > 0 {
                    final_segmented_sentences.push(previous_sentence.trim().to_string());
                }
                previous_sentence = sentence_candidate;
            }
        }

        if previous_sentence.len() > 0 {
            final_segmented_sentences.push(previous_sentence.trim().to_string());
        }

        final_segmented_sentences
    }

    pub fn amharic(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{1200}-\u{137F}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z\u{1200}-\u{137F}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z\u{1200}-\u{137F}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z\u{1200}-\u{137F}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z\u{1200}-\u{137F}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z\u{1200}-\u{137F}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z\u{1200}-\u{137F}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: "\u{1362}".to_string(),
            question_mark: "\u{1367}".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn afrikaans(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-Z])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-Z])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-Z])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-Z])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-Z])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-Z]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn albanian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZÇçËë\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-ZÇçËë])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-ZÇçËë])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-ZÇçËë])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-ZÇçËë])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-ZÇçËë])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-ZÇçËë]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn arabic(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0600}-\u{06FF}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-Z\u{0600}-\u{06FF}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-Z\u{0600}-\u{06FF}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-Z\u{0600}-\u{06FF}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-Z\u{0600}-\u{06FF}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-Z\u{0600}-\u{06FF}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"\b((?=[mdclxvi\u0621-\u064A])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\u{061F}(?=(\'|\"|[a-zA-Z\u{0600}-\u{06FF}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "\u{061F}".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["?".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn armenian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0531}-\u{058F}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z\u{0531}-\u{058F}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z\u{0531}-\u{058F}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z\u{0531}-\u{058F}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z\u{0531}-\u{058F}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z\u{0531}-\u{058F}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z\u{0531}-\u{058F}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: "\u{0589}".to_string(),
            question_mark: "\u{055E}".to_string(),
            exclamation_mark: "\u{055C}".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string(), ".".to_string(), "?".to_string(), "!".to_string(),],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn azerbaijani(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZÇŞĞÖÜİƏçşğıöüə\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-ZÇŞĞÖÜİƏçşğıöüə])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-ZÇŞĞÖÜİƏçşğıöüə])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-ZÇŞĞÖÜİƏçşğıöüə])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-ZÇŞĞÖÜİƏçşğıöüə])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-ZÇŞĞÖÜİƏçşğıöüə])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-ZÇŞĞÖÜİƏçşğıöüə]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn basque(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZÑÇñç\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-ZÑÇñç])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-ZÑÇñç])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-ZÑÇñç])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-ZÑÇñç])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-ZÑÇñç])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-ZÑÇñç]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn belarusian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZАБВГДЕЁЖЗІЙКЛМНОПРСТУФХЦЧШЫЬЭЮЯабвгдеёжзійклмнопрстуфхцчшыьэюя\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-ZАБВГДЕЁЖЗІЙКЛМНОПРСТУФХЦЧШЫЬЭЮЯабвгдеёжзійклмнопрстуфхцчшыьэюя])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-ZАБВГДЕЁЖЗІЙКЛМНОПРСТУФХЦЧШЫЬЭЮЯабвгдеёжзійклмнопрстуфхцчшыьэюя])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-ZАБВГДЕЁЖЗІЙКЛМНОПРСТУФХЦЧШЫЬЭЮЯабвгдеёжзійклмнопрстуфхцчшыьэюя])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-ZАБВГДЕЁЖЗІЙКЛМНОПРСТУФХЦЧШЫЬЭЮЯабвгдеёжзійклмнопрстуфхцчшыьэюя])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-ZАБВГДЕЁЖЗІЙКЛМНОПРСТУФХЦЧШЫЬЭЮЯабвгдеёжзійклмнопрстуфхцчшыьэюя])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-ZАБВГДЕЁЖЗІЙКЛМНОПРСТУФХЦЧШЫЬЭЮЯабвгдеёжзійклмнопрстуфхцчшыьэюя]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn bengali(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0980}-\u{09FF}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-Z\u{0980}-\u{09FF}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-Z\u{0980}-\u{09FF}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-Z\u{0980}-\u{09FF}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-Z\u{0980}-\u{09FF}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-Z\u{0980}-\u{09FF}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-Z\u{0980}-\u{09FF}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: "\u{0964}".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), ".".to_string(),],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn bosnian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZČĆDžĐŠŽčćdžđšž\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-ZČĆDžĐŠŽčćdžđšž])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-ZČĆDžĐŠŽčćdžđšž])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-ZČĆDžĐŠŽčćdžđšž])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-ZČĆDžĐŠŽčćdžđšž])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-ZČĆDžĐŠŽčćdžđšž])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-ZČĆDžĐŠŽčćdžđšž]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn bulgarian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn burmese(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{1000}-\u{109F}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z\u{1000}-\u{109F}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z\u{1000}-\u{109F}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z\u{1000}-\u{109F}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z\u{1000}-\u{109F}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z\u{1000}-\u{109F}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z\u{1000}-\u{109F}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: "\u{104B}".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn catalan(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZÀÁÂÈÉÊÌÍÒÓÔÙÚÜÇàáâèéêìíòóôùúüç\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-ZÀÁÂÈÉÊÌÍÒÓÔÙÚÜÇàáâèéêìíòóôùúüç])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-ZÀÁÂÈÉÊÌÍÒÓÔÙÚÜÇàáâèéêìíòóôùúüç])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-ZÀÁÂÈÉÊÌÍÒÓÔÙÚÜÇàáâèéêìíòóôùúüç])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-ZÀÁÂÈÉÊÌÍÒÓÔÙÚÜÇàáâèéêìíòóôùúüç])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-ZÀÁÂÈÉÊÌÍÒÓÔÙÚÜÇàáâèéêìíòóôùúüç])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-ZÀÁÂÈÉÊÌÍÒÓÔÙÚÜÇàáâèéêìíòóôùúüç]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn chinese(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{4E00}-\u{9FFF}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-Z\u{4E00}-\u{9FFF}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-Z\u{4E00}-\u{9FFF}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-Z\u{4E00}-\u{9FFF}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-Z\u{4E00}-\u{9FFF}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-Z\u{4E00}-\u{9FFF}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-Z\u{4E00}-\u{9FFF}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn croatian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZČĆŽŠĐčćžšđ\u{0400}-\u{042F}\u{0430}-\u{044F}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-ZČĆŽŠĐčćžšđ\u{0400}-\u{042F}\u{0430}-\u{044F}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-ZČĆŽŠĐčćžšđ\u{0400}-\u{042F}\u{0430}-\u{044F}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-ZČĆŽŠĐčćžšđ\u{0400}-\u{042F}\u{0430}-\u{044F}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-ZČĆŽŠĐčćžšđ\u{0400}-\u{042F}\u{0430}-\u{044F}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-ZČĆŽŠĐčćžšđ\u{0400}-\u{042F}\u{0430}-\u{044F}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-ZČĆŽŠĐčćžšđ\u{0400}-\u{042F}\u{0430}-\u{044F}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn czech(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZĚŠČŘŽÝÁÍÉÚŮÓěščřžýáíéúůó\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-ZĚŠČŘŽÝÁÍÉÚŮÓěščřžýáíéúůó])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-ZĚŠČŘŽÝÁÍÉÚŮÓěščřžýáíéúůó])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-ZĚŠČŘŽÝÁÍÉÚŮÓěščřžýáíéúůó])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-ZĚŠČŘŽÝÁÍÉÚŮÓěščřžýáíéúůó])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-ZĚŠČŘŽÝÁÍÉÚŮÓěščřžýáíéúůó])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-ZĚŠČŘŽÝÁÍÉÚŮÓěščřžýáíéúůó]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn danish(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZØÆÅøæå\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-ZØÆÅøæå])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-ZØÆÅøæå])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-ZØÆÅøæå])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-ZØÆÅøæå])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-ZØÆÅøæå])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-ZØÆÅøæå]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn dutch(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZÄËÏÖÜÇÀÈÌÒÙäëïöüçàèìòù\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-ZÄËÏÖÜÇÀÈÌÒÙäëïöüçàèìòù])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-ZÄËÏÖÜÇÀÈÌÒÙäëïöüçàèìòù])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-ZÄËÏÖÜÇÀÈÌÒÙäëïöüçàèìòù])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-ZÄËÏÖÜÇÀÈÌÒÙäëïöüçàèìòù])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-ZÄËÏÖÜÇÀÈÌÒÙäëïöüçàèìòù])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-ZÄËÏÖÜÇÀÈÌÒÙäëïöüçàèìòù]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn english(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn esperanto(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZĈĉĜĝĤĥĴĵŜŝŬŭ\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zĈĉĜĝĤĥĴĵŜŝŬŭ])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zĈĉĜĝĤĥĴĵŜŝŬŭ])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zĈĉĜĝĤĥĴĵŜŝŬŭ])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zĈĉĜĝĤĥĴĵŜŝŬŭ])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zĈĉĜĝĤĥĴĵŜŝŬŭ])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zĈĉĜĝĤĥĴĵŜŝŬŭ]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn estonian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZŠŽÕÄÖÜšžõäöü\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-ZŠŽÕÄÖÜšžõäöü])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-ZŠŽÕÄÖÜšžõäöü])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-ZŠŽÕÄÖÜšžõäöü])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-ZŠŽÕÄÖÜšžõäöü])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-ZŠŽÕÄÖÜšžõäöü])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-ZŠŽÕÄÖÜšžõäöü]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn finnish(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZÄÖÅäöå\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zÄÖÅäöå])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zÄÖÅäöå])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zÄÖÅäöå])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zÄÖÅäöå])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zÄÖÅäöå])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zÄÖÅäöå]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn french(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZÀÂÆÇÉÈÊËÎÏÔŒÙÛÜŸàâæçéèêëîïôœùûüÿ\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zÀÂÆÇÉÈÊËÎÏÔŒÙÛÜŸàâæçéèêëîïôœùûüÿ])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zÀÂÆÇÉÈÊËÎÏÔŒÙÛÜŸàâæçéèêëîïôœùûüÿ])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zÀÂÆÇÉÈÊËÎÏÔŒÙÛÜŸàâæçéèêëîïôœùûüÿ])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zÀÂÆÇÉÈÊËÎÏÔŒÙÛÜŸàâæçéèêëîïôœùûüÿ])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zÀÂÆÇÉÈÊËÎÏÔŒÙÛÜŸàâæçéèêëîïôœùûüÿ])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zÀÂÆÇÉÈÊËÎÏÔŒÙÛÜŸàâæçéèêëîïôœùûüÿ]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn ganda(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZÀÈÙÁÉÍÓÚàèùáéíóú\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zÀÈÙÁÉÍÓÚàèùáéíóú])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zÀÈÙÁÉÍÓÚàèùáéíóú])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zÀÈÙÁÉÍÓÚàèùáéíóú])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zÀÈÙÁÉÍÓÚàèùáéíóú])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zÀÈÙÁÉÍÓÚàèùáéíóú])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zÀÈÙÁÉÍÓÚàèùáéíóú]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn georgian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{10A0}-\u{10FF}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z\u{10A0}-\u{10FF}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z\u{10A0}-\u{10FF}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z\u{10A0}-\u{10FF}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z\u{10A0}-\u{10FF}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z\u{10A0}-\u{10FF}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z\u{10A0}-\u{10FF}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn german(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZÄÖÜẞäöüß\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zÄÖÜẞäöüß])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zÄÖÜẞäöüß])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zÄÖÜẞäöüß])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zÄÖÜẞäöüß])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zÄÖÜẞäöüß])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zÄÖÜẞäöüß]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn greek(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZάέήίόύώΆΈΉΊΌΎΏ\u{0391}-\u{03A9}\u{03B1}-\u{03C9}\u{03C2}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zάέήίόύώΆΈΉΊΌΎΏ\u{0391}-\u{03A9}\u{03B1}-\u{03C9}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zάέήίόύώΆΈΉΊΌΎΏ\u{0391}-\u{03A9}\u{03B1}-\u{03C9}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zάέήίόύώΆΈΉΊΌΎΏ\u{0391}-\u{03A9}\u{03B1}-\u{03C9}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zάέήίόύώΆΈΉΊΌΎΏ\u{0391}-\u{03A9}\u{03B1}-\u{03C9}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zάέήίόύώΆΈΉΊΌΎΏ\u{0391}-\u{03A9}\u{03B1}-\u{03C9}])*’").unwrap();        
        let roman_numerals_regex = Regex::new(r"\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zάέήίόύώΆΈΉΊΌΎΏ\u{0391}-\u{03A9}\u{03B1}-\u{03C9}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn gujarati(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0A80}-\u{0AFF}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z\u{0A80}-\u{0AFF}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z\u{0A80}-\u{0AFF}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z\u{0A80}-\u{0AFF}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z\u{0A80}-\u{0AFF}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z\u{0A80}-\u{0AFF}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z\u{0A80}-\u{0AFF}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn hausa(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZƁɓƊɗƘƙ\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zƁɓƊɗƘƙ])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zƁɓƊɗƘƙ])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zƁɓƊɗƘƙ])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zƁɓƊɗƘƙ])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zƁɓƊɗƘƙ])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zƁɓƊɗƘƙ]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn hebrew(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0590}-\u{05FF}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-Z\u{0590}-\u{05FF}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-Z\u{0590}-\u{05FF}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-Z\u{0590}-\u{05FF}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-Z\u{0590}-\u{05FF}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-Z\u{0590}-\u{05FF}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"\b((?=[mdclxvi\u0621-\u064A])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\u{061F}(?=(\'|\"|[a-zA-Z\u{0590}-\u{05FF}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["?".to_string(), "。".to_string(), "\u{0964}".to_string(), "\u{061F}".to_string(),],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn hindi(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0900}-\u{097F}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z\u{0900}-\u{097F}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z\u{0900}-\u{097F}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z\u{0900}-\u{097F}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-Z\u{0900}-\u{097F}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-Z\u{0900}-\u{097F}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-Z\u{0900}-\u{097F}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: "\u{0964}".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn hungarian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZÁÉÍÓÖŐÚÜáéíóöőúü\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zÁÉÍÓÖŐÚÜáéíóöőúü])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zÁÉÍÓÖŐÚÜáéíóöőúü])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zÁÉÍÓÖŐÚÜáéíóöőúü])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zÁÉÍÓÖŐÚÜáéíóöőúü])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zÁÉÍÓÖŐÚÜáéíóöőúü])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zÁÉÍÓÖŐÚÜáéíóöőúü]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn icelandic(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZÁáÉéÍíÓóÚúÝýÞþÆæÖö\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zÁáÉéÍíÓóÚúÝýÞþÆæÖö])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zÁáÉéÍíÓóÚúÝýÞþÆæÖö])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zÁáÉéÍíÓóÚúÝýÞþÆæÖö])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zÁáÉéÍíÓóÚúÝýÞþÆæÖö])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zÁáÉéÍíÓóÚúÝýÞþÆæÖö])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zÁáÉéÍíÓóÚúÝýÞþÆæÖö]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn igbo(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZỊṄỌỤịṅọụ\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zỊṄỌỤịṅọụ])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zỊṄỌỤịṅọụ])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zỊṄỌỤịṅọụ])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zỊṄỌỤịṅọụ])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zỊṄỌỤịṅọụ])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zỊṄỌỤịṅọụ]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn indonesian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZÁáÉéÍíÓóÚúŃńÇçĐđ\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zÁáÉéÍíÓóÚúŃńÇçĐđ])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zÁáÉéÍíÓóÚúŃńÇçĐđ])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zÁáÉéÍíÓóÚúŃńÇçĐđ])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zÁáÉéÍíÓóÚúŃńÇçĐđ])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zÁáÉéÍíÓóÚúŃńÇçĐđ])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zÁáÉéÍíÓóÚúŃńÇçĐđ]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn irish(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZÁáÉéÍíÓóÚúÇç\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zÁáÉéÍíÓóÚúÇç])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zÁáÉéÍíÓóÚúÇç])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zÁáÉéÍíÓóÚúÇç])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zÁáÉéÍíÓóÚúÇç])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zÁáÉéÍíÓóÚúÇç])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zÁáÉéÍíÓóÚúÇç]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn italian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZÀÈÉÌÒÙàèéìòù\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zÀÈÉÌÒÙàèéìòù])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zÀÈÉÌÒÙàèéìòù])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zÀÈÉÌÒÙàèéìòù])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zÀÈÉÌÒÙàèéìòù])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zÀÈÉÌÒÙàèéìòù])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zÀÈÉÌÒÙàèéìòù]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn japanese(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{3040}-\u{309F}\u{30A0}-\u{30FF}\u{4E00}-\u{9FAF}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-Z\u{3040}-\u{309F}\u{30A0}-\u{30FF}\u{4E00}-\u{9FAF}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-Z\u{3040}-\u{309F}\u{30A0}-\u{30FF}\u{4E00}-\u{9FAF}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-Z\u{3040}-\u{309F}\u{30A0}-\u{30FF}\u{4E00}-\u{9FAF}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-Z\u{3040}-\u{309F}\u{30A0}-\u{30FF}\u{4E00}-\u{9FAF}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-Z\u{3040}-\u{309F}\u{30A0}-\u{30FF}\u{4E00}-\u{9FAF}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-Z\u{3040}-\u{309F}\u{30A0}-\u{30FF}\u{4E00}-\u{9FAF}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: "。".to_string(),
            question_mark: "？".to_string(),
            exclamation_mark: "！".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn kazakh(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZÁáÀàÂâÄäÇçEéFfGgHhÍíJjKkLlmMNnOóPpRrSsŞşTtUúVvYyZz\u{0400}-\u{04FF}\u{04D8}\u{0492}\u{042C}\u{04A2}\u{04A2}\u{04B0}\u{04AE}\u{04B1}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zÁáÀàÂâÄäÇçEéFfGgHhÍíJjKkLlmMNnOóPpRrSsŞşTtUúVvYyZz\u{0400}-\u{04FF}\u{04D8}\u{0492}\u{042C}\u{04A2}\u{04A2}\u{04B0}\u{04AE}\u{04B1}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zÁáÀàÂâÄäÇçEéFfGgHhÍíJjKkLlmMNnOóPpRrSsŞşTtUúVvYyZz\u{0400}-\u{04FF}\u{04D8}\u{0492}\u{042C}\u{04A2}\u{04A2}\u{04B0}\u{04AE}\u{04B1}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zÁáÀàÂâÄäÇçEéFfGgHhÍíJjKkLlmMNnOóPpRrSsŞşTtUúVvYyZz\u{0400}-\u{04FF}\u{04D8}\u{0492}\u{042C}\u{04A2}\u{04A2}\u{04B0}\u{04AE}\u{04B1}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zÁáÀàÂâÄäÇçEéFfGgHhÍíJjKkLlmMNnOóPpRrSsŞşTtUúVvYyZz\u{0400}-\u{04FF}\u{04D8}\u{0492}\u{042C}\u{04A2}\u{04A2}\u{04B0}\u{04AE}\u{04B1}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zÁáÀàÂâÄäÇçEéFfGgHhÍíJjKkLlmMNnOóPpRrSsŞşTtUúVvYyZz\u{0400}-\u{04FF}\u{04D8}\u{0492}\u{042C}\u{04A2}\u{04A2}\u{04B0}\u{04AE}\u{04B1}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zÁáÀàÂâÄäÇçEéFfGgHhÍíJjKkLlmMNnOóPpRrSsŞşTtUúVvYyZz\u{0400}-\u{04FF}\u{04D8}\u{0492}\u{042C}\u{04A2}\u{04A2}\u{04B0}\u{04AE}\u{04B1}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn khmer(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{1780}-\u{17FF}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z\u{1780}-\u{17FF}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z\u{1780}-\u{17FF}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z\u{1780}-\u{17FF}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z\u{1780}-\u{17FF}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z\u{1780}-\u{17FF}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z\u{1780}-\u{17FF}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: "\u{17D4}".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "\u{17D5}".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string(), "!".to_string(), "?".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn korean(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{AC00}-\u{D7A3}\u{4E00}-\u{9FAF}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-Z\u{AC00}-\u{D7A3}\u{4E00}-\u{9FAF}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-Z\u{AC00}-\u{D7A3}\u{4E00}-\u{9FAF}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-Z\u{AC00}-\u{D7A3}\u{4E00}-\u{9FAF}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-Z\u{AC00}-\u{D7A3}\u{4E00}-\u{9FAF}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-Z\u{AC00}-\u{D7A3}\u{4E00}-\u{9FAF}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-Z\u{AC00}-\u{D7A3}\u{4E00}-\u{9FAF}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn kurdish(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZÇÊÎŞÛçêîşû\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zÇÊÎŞÛçêîşû])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zÇÊÎŞÛçêîşû])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zÇÊÎŞÛçêîşû])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zÇÊÎŞÛçêîşû])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zÇÊÎŞÛçêîşû])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zÇÊÎŞÛçêîşû]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn kyrgyz(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0400}-\u{04FF}\u{04E8}\u{04E9}\u{04AE}\u{04AF}\u{04A8}\u{04A9}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z\u{0400}-\u{04FF}\u{04E8}\u{04E9}\u{04AE}\u{04AF}\u{04A8}\u{04A9}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z\u{0400}-\u{04FF}\u{04E8}\u{04E9}\u{04AE}\u{04AF}\u{04A8}\u{04A9}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z\u{0400}-\u{04FF}\u{04E8}\u{04E9}\u{04AE}\u{04AF}\u{04A8}\u{04A9}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z\u{0400}-\u{04FF}\u{04E8}\u{04E9}\u{04AE}\u{04AF}\u{04A8}\u{04A9}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z\u{0400}-\u{04FF}\u{04E8}\u{04E9}\u{04AE}\u{04AF}\u{04A8}\u{04A9}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z\u{0400}-\u{04FF}\u{04E8}\u{04E9}\u{04AE}\u{04AF}\u{04A8}\u{04A9}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn lao(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0E80}-\u{0EFF}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z\u{0E80}-\u{0EFF}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z\u{0E80}-\u{0EFF}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z\u{0E80}-\u{0EFF}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z\u{0E80}-\u{0EFF}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z\u{0E80}-\u{0EFF}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z\u{0E80}-\u{0EFF}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn latin(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn latvian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZĀČĒĢĪĶĻŅŌŖŠŪŽāčēģīķļņōŗšūž\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zĀČĒĢĪĶĻŅŌŖŠŪŽāčēģīķļņōŗšūž])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zĀČĒĢĪĶĻŅŌŖŠŪŽāčēģīķļņōŗšūž])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zĀČĒĢĪĶĻŅŌŖŠŪŽāčēģīķļņōŗšūž])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zĀČĒĢĪĶĻŅŌŖŠŪŽāčēģīķļņōŗšūž])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zĀČĒĢĪĶĻŅŌŖŠŪŽāčēģīķļņōŗšūž])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zĀČĒĢĪĶĻŅŌŖŠŪŽāčēģīķļņōŗšūž]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn lithuanian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZĄČĘĖĮŠŲŪŽąčęėįšųūž\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zĄČĘĖĮŠŲŪŽąčęėįšųūž])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zĄČĘĖĮŠŲŪŽąčęėįšųūž])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zĄČĘĖĮŠŲŪŽąčęėįšųūž])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zĄČĘĖĮŠŲŪŽąčęėįšųūž])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zĄČĘĖĮŠŲŪŽąčęėįšųūž])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zĄČĘĖĮŠŲŪŽąčęėįšųūž]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn macedonian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0405}\u{0455}\u{0408}\u{0458}\u{0409}\u{0459}\u{040A}\u{045A}\u{040C}\u{045C}\u{040F}\u{045F}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0405}\u{0455}\u{0408}\u{0458}\u{0409}\u{0459}\u{040A}\u{045A}\u{040C}\u{045C}\u{040F}\u{045F}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0405}\u{0455}\u{0408}\u{0458}\u{0409}\u{0459}\u{040A}\u{045A}\u{040C}\u{045C}\u{040F}\u{045F}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0405}\u{0455}\u{0408}\u{0458}\u{0409}\u{0459}\u{040A}\u{045A}\u{040C}\u{045C}\u{040F}\u{045F}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0405}\u{0455}\u{0408}\u{0458}\u{0409}\u{0459}\u{040A}\u{045A}\u{040C}\u{045C}\u{040F}\u{045F}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0405}\u{0455}\u{0408}\u{0458}\u{0409}\u{0459}\u{040A}\u{045A}\u{040C}\u{045C}\u{040F}\u{045F}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0405}\u{0455}\u{0408}\u{0458}\u{0409}\u{0459}\u{040A}\u{045A}\u{040C}\u{045C}\u{040F}\u{045F}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    // basically same as indonesian
    pub fn malay(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZÁáÉéÍíÓóÚúŃńÇçĐđ\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zÁáÉéÍíÓóÚúŃńÇçĐđ])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zÁáÉéÍíÓóÚúŃńÇçĐđ])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zÁáÉéÍíÓóÚúŃńÇçĐđ])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zÁáÉéÍíÓóÚúŃńÇçĐđ])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zÁáÉéÍíÓóÚúŃńÇçĐđ])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zÁáÉéÍíÓóÚúŃńÇçĐđ]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn maori(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZĀāĒēĪīŌōŪū\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zĀāĒēĪīŌōŪū])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zĀāĒēĪīŌōŪū])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zĀāĒēĪīŌōŪū])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zĀāĒēĪīŌōŪū])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zĀāĒēĪīŌōŪū])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zĀāĒēĪīŌōŪū]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn marathi(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0900}-\u{097F}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z\u{0900}-\u{097F}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z\u{0900}-\u{097F}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z\u{0900}-\u{097F}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z\u{0900}-\u{097F}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z\u{0900}-\u{097F}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z\u{0900}-\u{097F}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn mongolian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{04E8}\u{04E9}\u{04AE}\u{04AF}\u{1800}-\u{18AF}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{04E8}\u{04E9}\u{04AE}\u{04AF}\u{1800}-\u{18AF}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{04E8}\u{04E9}\u{04AE}\u{04AF}\u{1800}-\u{18AF}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{04E8}\u{04E9}\u{04AE}\u{04AF}\u{1800}-\u{18AF}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{04E8}\u{04E9}\u{04AE}\u{04AF}\u{1800}-\u{18AF}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{04E8}\u{04E9}\u{04AE}\u{04AF}\u{1800}-\u{18AF}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{04E8}\u{04E9}\u{04AE}\u{04AF}\u{1800}-\u{18AF}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn nepali(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0900}-\u{097F}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z\u{0900}-\u{097F}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z\u{0900}-\u{097F}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z\u{0900}-\u{097F}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z\u{0900}-\u{097F}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z\u{0900}-\u{097F}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z\u{0900}-\u{097F}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: "\u{0964}".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn norwegian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}A-Za-zÆØÅæøå\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zÆØÅæøå])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zÆØÅæøå])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zÆØÅæøå])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zÆØÅæøå])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zÆØÅæøå])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zÆØÅæøå]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn pashto(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0600}-\u{06FF}\u{0750}-\u{077F}\u{FB50}-\u{FDFF}\u{FE70}-\u{FEFF}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z\u{0600}-\u{06FF}\u{0750}-\u{077F}\u{FB50}-\u{FDFF}\u{FE70}-\u{FEFF}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z\u{0600}-\u{06FF}\u{0750}-\u{077F}\u{FB50}-\u{FDFF}\u{FE70}-\u{FEFF}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z\u{0600}-\u{06FF}\u{0750}-\u{077F}\u{FB50}-\u{FDFF}\u{FE70}-\u{FEFF}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z\u{0600}-\u{06FF}\u{0750}-\u{077F}\u{FB50}-\u{FDFF}\u{FE70}-\u{FEFF}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z\u{0600}-\u{06FF}\u{0750}-\u{077F}\u{FB50}-\u{FDFF}\u{FE70}-\u{FEFF}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z\u{0600}-\u{06FF}\u{0750}-\u{077F}\u{FB50}-\u{FDFF}\u{FE70}-\u{FEFF}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "\u{061F}".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["?".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn persian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0600}-\u{06FF}\u{0686}\u{06AF}\u{06A9}\u{0698}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-Z\u{0600}-\u{06FF}\u{0686}\u{06AF}\u{06A9}\u{0698}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-Z\u{0600}-\u{06FF}\u{0686}\u{06AF}\u{06A9}\u{0698}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-Z\u{0600}-\u{06FF}\u{0686}\u{06AF}\u{06A9}\u{0698}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-Z\u{0600}-\u{06FF}\u{0686}\u{06AF}\u{06A9}\u{0698}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-Z\u{0600}-\u{06FF}\u{0686}\u{06AF}\u{06A9}\u{0698}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\u{061F}(?=(\'|\"|[a-zA-Z\u{0600}-\u{06FF}\u{0686}\u{06AF}\u{06A9}\u{0698}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "\u{061F}".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["?".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn polish(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}A-Za-zĄĆĘŁŃÓŚŹŻąćęłńóśźż\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zĄĆĘŁŃÓŚŹŻąćęłńóśźż])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zĄĆĘŁŃÓŚŹŻąćęłńóśźż])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zĄĆĘŁŃÓŚŹŻąćęłńóśźż])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zĄĆĘŁŃÓŚŹŻąćęłńóśźż])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zĄĆĘŁŃÓŚŹŻąćęłńóśźż])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zĄĆĘŁŃÓŚŹŻąćęłńóśźż]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn portuguese(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}A-Za-zÀÁÂÃÄÇÉÊËÍÎÏÓÔÕÖÚÛÜàáâãäçéêëíîïóôõöúûü\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zÀÁÂÃÄÇÉÊËÍÎÏÓÔÕÖÚÛÜàáâãäçéêëíîïóôõöúûü])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zÀÁÂÃÄÇÉÊËÍÎÏÓÔÕÖÚÛÜàáâãäçéêëíîïóôõöúûü])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zÀÁÂÃÄÇÉÊËÍÎÏÓÔÕÖÚÛÜàáâãäçéêëíîïóôõöúûü])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zÀÁÂÃÄÇÉÊËÍÎÏÓÔÕÖÚÛÜàáâãäçéêëíîïóôõöúûü])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zÀÁÂÃÄÇÉÊËÍÎÏÓÔÕÖÚÛÜàáâãäçéêëíîïóôõöúûü])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zÀÁÂÃÄÇÉÊËÍÎÏÓÔÕÖÚÛÜàáâãäçéêëíîïóôõöúûü]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn punjabi_eastern(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0900}-\u{097F}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z\u{0900}-\u{097F}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z\u{0900}-\u{097F}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z\u{0900}-\u{097F}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-Z\u{0900}-\u{097F}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-Z\u{0900}-\u{097F}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-Z\u{0900}-\u{097F}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: "\u{0964}".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn punjabi_western(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0600}-\u{06D4}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-Z\u{0600}-\u{06D4}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-Z\u{0600}-\u{06D4}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-Z\u{0600}-\u{06D4}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-Z\u{0600}-\u{06D4}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-Z\u{0600}-\u{06D4}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"\b((?=[mdclxvi\u0621-\u064A])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\u{061F}(?=(\'|\"|[a-zA-Z\u{0600}-\u{06D4}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "\u{061F}".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["?".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn romanian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}A-Za-zĂÂÎȘȚăâîșț\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zĂÂÎȘȚăâîșț])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zĂÂÎȘȚăâîșț])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zĂÂÎȘȚăâîșț])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zĂÂÎȘȚăâîșț])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zĂÂÎȘȚăâîșț])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zĂÂÎȘȚăâîșț]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn russian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0401}\u{0451}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0401}\u{0451}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0401}\u{0451}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0401}\u{0451}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0401}\u{0451}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0401}\u{0451}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0401}\u{0451}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn scottish(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn serbian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZščćđžŠČĆĐŽ\u{0400}-\u{04FF}\u{0459}\u{0409}\u{045A}\u{040A}\u{045B}\u{040B}\u{0452}\u{0402}\u{0436}\u{0406}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zščćđžŠČĆĐŽ\u{0400}-\u{04FF}\u{0459}\u{0409}\u{045A}\u{040A}\u{045B}\u{040B}\u{0452}\u{0402}\u{0436}\u{0406}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zščćđžŠČĆĐŽ\u{0400}-\u{04FF}\u{0459}\u{0409}\u{045A}\u{040A}\u{045B}\u{040B}\u{0452}\u{0402}\u{0436}\u{0406}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zščćđžŠČĆĐŽ\u{0400}-\u{04FF}\u{0459}\u{0409}\u{045A}\u{040A}\u{045B}\u{040B}\u{0452}\u{0402}\u{0436}\u{0406}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zščćđžŠČĆĐŽ\u{0400}-\u{04FF}\u{0459}\u{0409}\u{045A}\u{040A}\u{045B}\u{040B}\u{0452}\u{0402}\u{0436}\u{0406}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zščćđžŠČĆĐŽ\u{0400}-\u{04FF}\u{0459}\u{0409}\u{045A}\u{040A}\u{045B}\u{040B}\u{0452}\u{0402}\u{0436}\u{0406}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zščćđžŠČĆĐŽ\u{0400}-\u{04FF}\u{0459}\u{0409}\u{045A}\u{040A}\u{045B}\u{040B}\u{0452}\u{0402}\u{0436}\u{0406}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn shona(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZáÁâÂàÀéÉèÈíÍìÌóÓòÒúÚùÙñÑ\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-záÁâÂàÀéÉèÈíÍìÌóÓòÒúÚùÙñÑ])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-záÁâÂàÀéÉèÈíÍìÌóÓòÒúÚùÙñÑ])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-záÁâÂàÀéÉèÈíÍìÌóÓòÒúÚùÙñÑ])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-záÁâÂàÀéÉèÈíÍìÌóÓòÒúÚùÙñÑ])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-záÁâÂàÀéÉèÈíÍìÌóÓòÒúÚùÙñÑ])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-záÁâÂàÀéÉèÈíÍìÌóÓòÒúÚùÙñÑ]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn sinhala(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0D80}-\u{0DFF}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z\u{0D80}-\u{0DFF}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z\u{0D80}-\u{0DFF}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z\u{0D80}-\u{0DFF}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z\u{0D80}-\u{0DFF}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z\u{0D80}-\u{0DFF}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z\u{0D80}-\u{0DFF}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn slovak(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}A-Za-zÁÄČĎÉĽÍŇÓÔÚÝáäčďéľíňóôúý\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zÁÄČĎÉĽÍŇÓÔÚÝáäčďéľíňóôúý])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zÁÄČĎÉĽÍŇÓÔÚÝáäčďéľíňóôúý])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zÁÄČĎÉĽÍŇÓÔÚÝáäčďéľíňóôúý])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zÁÄČĎÉĽÍŇÓÔÚÝáäčďéľíňóôúý])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zÁÄČĎÉĽÍŇÓÔÚÝáäčďéľíňóôúý])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zÁÄČĎÉĽÍŇÓÔÚÝáäčďéľíňóôúý]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn slovenian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}A-Za-zČŠŽčšž\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zČŠŽčšž])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zČŠŽčšž])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zČŠŽčšž])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zČŠŽčšž])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zČŠŽčšž])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zČŠŽčšž]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn somali(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZÀàÁáÂâÆæÈèÉéÊêËëÍíÎîÒòÓóÔôÙùÚúÛû\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zÀàÁáÂâÆæÈèÉéÊêËëÍíÎîÒòÓóÔôÙùÚúÛû])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zÀàÁáÂâÆæÈèÉéÊêËëÍíÎîÒòÓóÔôÙùÚúÛû])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zÀàÁáÂâÆæÈèÉéÊêËëÍíÎîÒòÓóÔôÙùÚúÛû])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zÀàÁáÂâÆæÈèÉéÊêËëÍíÎîÒòÓóÔôÙùÚúÛû])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zÀàÁáÂâÆæÈèÉéÊêËëÍíÎîÒòÓóÔôÙùÚúÛû])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zÀàÁáÂâÆæÈèÉéÊêËëÍíÎîÒòÓóÔôÙùÚúÛû]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn sotho(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZÀàÁáÈèÉéÊêÍíÒòÓóÔôÙùÚú\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zÀàÁáÈèÉéÊêÍíÒòÓóÔôÙùÚú])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zÀàÁáÈèÉéÊêÍíÒòÓóÔôÙùÚú])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zÀàÁáÈèÉéÊêÍíÒòÓóÔôÙùÚú])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zÀàÁáÈèÉéÊêÍíÒòÓóÔôÙùÚú])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zÀàÁáÈèÉéÊêÍíÒòÓóÔôÙùÚú])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zÀàÁáÈèÉéÊêÍíÒòÓóÔôÙùÚú]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn spanish(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}A-Za-zÁÉÍÓÚÑáéíóúñ\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zÁÉÍÓÚÑáéíóúñ])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zÁÉÍÓÚÑáéíóúñ])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zÁÉÍÓÚÑáéíóúñ])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zÁÉÍÓÚÑáéíóúñ])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zÁÉÍÓÚÑáéíóúñ])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zÁÉÍÓÚÑáéíóúñ]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn swahili(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn swedish(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}A-Za-zÅÄÖåäö\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zÅÄÖåäö])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zÅÄÖåäö])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zÅÄÖåäö])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zÅÄÖåäö])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zÅÄÖåäö])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zÅÄÖåäö]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn tagalog(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn tamil(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}A-Za-z\u{0B80}-\u{0BFF}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z\u{0B80}-\u{0BFF}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z\u{0B80}-\u{0BFF}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z\u{0B80}-\u{0BFF}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z\u{0B80}-\u{0BFF}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z\u{0B80}-\u{0BFF}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z\u{0B80}-\u{0BFF}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn telugu(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}A-Za-z\u{0C00}-\u{0C7F}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z\u{0C00}-\u{0C7F}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z\u{0C00}-\u{0C7F}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z\u{0C00}-\u{0C7F}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z\u{0C00}-\u{0C7F}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z\u{0C00}-\u{0C7F}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z\u{0C00}-\u{0C7F}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn tsonga(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn tswana(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn turkish(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}A-Za-zĞğİıÇçŞşÖöÜü\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zĞğİıÇçŞşÖöÜü])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zĞğİıÇçŞşÖöÜü])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zĞğİıÇçŞşÖöÜü])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zĞğİıÇçŞşÖöÜü])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zĞğİıÇçŞşÖöÜü])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zĞğİıÇçŞşÖöÜü]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn ukrainian(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0404}\u{0454}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0404}\u{0454}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0404}\u{0454}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0404}\u{0454}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0404}\u{0454}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0404}\u{0454}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[a-zA-Z\u{0410}-\u{042F}\u{0430}-\u{044F}\u{0404}\u{0454}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn urdu(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\u{0600}-\u{06FF}\u{067E}\u{067F}\u{0686}\u{0691}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[a-zA-Z\u{0600}-\u{06FF}\u{067E}\u{067F}\u{0686}\u{0691}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[a-zA-Z\u{0600}-\u{06FF}\u{067E}\u{067F}\u{0686}\u{0691}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[a-zA-Z\u{0600}-\u{06FF}\u{067E}\u{067F}\u{0686}\u{0691}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[a-zA-Z\u{0600}-\u{06FF}\u{067E}\u{067F}\u{0686}\u{0691}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[a-zA-Z\u{0600}-\u{06FF}\u{067E}\u{067F}\u{0686}\u{0691}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"\b((?=[mdclxvi\u0621-\u064A])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\u{061F}(?=(\'|\"|[a-zA-Z\u{0600}-\u{06FF}\u{067E}\u{067F}\u{0686}\u{0691}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "\u{061F}".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["?".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn uzbek(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZŌŞÇḠōşçḡ\u{0400}-\u{04FF}\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zŌŞÇḠōşçḡ\u{0400}-\u{04FF}])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zŌŞÇḠōşçḡ\u{0400}-\u{04FF}])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zŌŞÇḠōşçḡ\u{0400}-\u{04FF}])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zŌŞÇḠōşçḡ\u{0400}-\u{04FF}])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zŌŞÇḠōşçḡ\u{0400}-\u{04FF}])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zŌŞÇḠōşçḡ\u{0400}-\u{04FF}]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn vietnamese(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}A-Za-zÀÁÂÃÈÉÊÌÍÒÓÔÕÙÚĂĐĨŨƠƯàáâãèéêìíòóôõùúăđĩũơưỲÝỴỶỸỳýỵỷỹ\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zÀÁÂÃÈÉÊÌÍÒÓÔÕÙÚĂĐĨŨƠƯàáâãèéêìíòóôõùúăđĩũơưỲÝỴỶỸỳýỵỷỹ])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zÀÁÂÃÈÉÊÌÍÒÓÔÕÙÚĂĐĨŨƠƯàáâãèéêìíòóôõùúăđĩũơưỲÝỴỶỸỳýỵỷỹ])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zÀÁÂÃÈÉÊÌÍÒÓÔÕÙÚĂĐĨŨƠƯàáâãèéêìíòóôõùúăđĩũơưỲÝỴỶỸỳýỵỷỹ])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zÀÁÂÃÈÉÊÌÍÒÓÔÕÙÚĂĐĨŨƠƯàáâãèéêìíòóôõùúăđĩũơưỲÝỴỶỸỳýỵỷỹ])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zÀÁÂÃÈÉÊÌÍÒÓÔÕÙÚĂĐĨŨƠƯàáâãèéêìíòóôõùúăđĩũơưỲÝỴỶỸỳýỵỷỹ])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zÀÁÂÃÈÉÊÌÍÒÓÔÕÙÚĂĐĨŨƠƯàáâãèéêìíòóôõùúăđĩũơưỲÝỴỶỸỳýỵỷỹ]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn welsh(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn xhosa(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn yoruba(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-ZẸỌṢẹọṣ\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-zẸỌṢẹọṣ])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-zẸỌṢẹọṣ])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-zẸỌṢẹọṣ])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-zẸỌṢẹọṣ])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-zẸỌṢẹọṣ])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-zẸỌṢẹọṣ]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }

    pub fn zulu(text: &str) -> Vec<String> {
        let alphabet_regex = Regex::new(r"[^0-9\u{17E0}-\u{17E9}\u{1040}-\u{1049}\u{0660}-\u{0669}a-zA-Z\s\?!:.،。、·\u{0ECD}\u{0ECC}\u{0EAF}\u{17D4}\u{17D5}\u{17D6}\u{104A}\u{104B}\u{00BF}\u{00A1}\u{1362}\u{1367}\u{0589}\u{055D}\u{055C}\u{055E}\u{061F}\u{0964}\u{0965}\\「\\」\\)\\(\\[\\]\\-_]").unwrap();
        let three_consecutive_rule = Regex::new(r"\.\.\.(?=\s+[A-Za-z])").unwrap();
        let four_consecutive_rule = Regex::new(r"(?<=\S)\.{3}(?=\.\s[A-Za-z])").unwrap();
        let four_space_rule = Regex::new(r"(?<=[A-Za-z])(\.\s){3}\.(\z|$|\n)").unwrap();
        let between_single_quotes_regex = Regex::new(r"(?<=\s)'(?:[^']|'[A-Za-z])*'").unwrap();
        let between_single_quote_slanted_regex = Regex::new(r"(?<=\s)‘(?:[^’]|’[A-Za-z])*’").unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        let question_mark_in_quotation_regex = Regex::new(r#"\?(?=(\'|\"|[A-Za-z]))"#).unwrap();
        let punc_config = PunctuationConfig {
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "。".to_string(), "\u{0964}".to_string()],
        };
        let config = LanguageConfig {
            alphabet_regex: alphabet_regex,
            three_consecutive_rule: three_consecutive_rule,
            four_consecutive_rule: four_consecutive_rule,
            four_space_rule: four_space_rule,
            between_single_quotes_regex: between_single_quotes_regex,
            between_single_quote_slanted_regex: between_single_quote_slanted_regex,
            roman_numerals_regex: roman_numerals_regex,
            question_mark_in_quotation_regex: question_mark_in_quotation_regex,
            punctuation_config: punc_config,
        };

        process(text, config)
    }
}