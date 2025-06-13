const ABBREVIATION_MAP_JSON: &str = include_str!("abbreviation_map.json");

pub mod processor {
    use super::*;
    // use std::fs;
    use serde_json::Value;
    use std::collections::HashMap;
    use fancy_regex::Regex;
    use regex::Regex as SecondRegex;

    struct LanguageConfig {
        alphabets: String,
        have_capital_letter: bool,
        period: String,
        question_mark: String,
        exclamation_mark: String,
        other_punctuations: Vec<String>,
    }

    fn process(text: &str, config: LanguageConfig) -> Vec<String> {
        // step 1 : remove redundant \n, \t, \r and \s+
        let redundant_space_rule = Regex::new(r"\s+").unwrap();
        let mut filtered_string = redundant_space_rule.replace_all(&text, " ").to_string();
        let new_line_rule = Regex::new(r"\n").unwrap();
        filtered_string = new_line_rule.replace_all(&filtered_string, " ").to_string();
        let tab_rule = Regex::new(r"\t").unwrap();
        filtered_string = tab_rule.replace_all(&filtered_string, " ").to_string();
        let carriage_return_rule = Regex::new(r"\r").unwrap();
        filtered_string = carriage_return_rule.replace_all(&filtered_string, " ").to_string();

        let all_punctuations = "¿¡、，\u{0021}\u{002E}\u{003F}\u{0589}\u{061F}\u{06D4}\u{0700}\u{0701}\u{0702}\u{07F9}\u{0964}\u{0965}\u{104A}\u{104B}\u{1362}\u{1367}\u{1368}\u{166E}\u{1803}\u{1809}\u{1944}\u{1945}\u{1AA8}\u{1AA9}\u{1AAA}\u{1AAB}\u{1B5A}\u{1B5B}\u{1B5E}\u{1B5F}\u{1C3B}\u{1C3C}\u{1C7E}\u{1C7F}\u{203C}\u{203D}\u{2047}\u{2048}\u{2049}\u{2E2E}\u{3002}\u{A4FF}\u{A60E}\u{A60F}\u{A6F3}\u{A6F7}\u{A876}\u{A877}\u{A8CE}\u{A8CF}\u{A92F}\u{A9C8}\u{A9C9}\u{AA5D}\u{AA5E}\u{AA5F}\u{AAF0}\u{AAF1}\u{ABEB}\u{FE52}\u{FE56}\u{FE57}\u{FF01}\u{FF0E}\u{FF1F}\u{FF61}\u{11047}\u{11048}\u{110BE}\u{110BF}\u{110C0}\u{110C1}\u{11141}\u{11142}\u{11143}\u{111C5}\u{111C6}".to_string();

        // step 2 : eliminate non-alphabet
        let alphabet_regex_pattern = format!(
            r"[^0-9\u{{A9D0}}-\u{{A9D9}}\u{{17E0}}-\u{{17E9}}\u{{1040}}-\u{{1049}}\u{{0660}}-\u{{0669}}{}\s\{}\\「\\」\\)\\(\\[\\]\\-_]",
            config.alphabets.clone(),
            all_punctuations.clone()
        );
        let alphabet_regex = Regex::new(&alphabet_regex_pattern).unwrap();
        filtered_string = alphabet_regex.replace_all(&filtered_string, "").to_string();

        // step 3 : remove numbered list (ex 1., 2., ...)
        let numbered_list_regex = Regex::new(r"\d+\.\s*").unwrap();
        filtered_string = numbered_list_regex.replace_all(&filtered_string, " ").to_string();

        // step 4 : mask abbreviations
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

        // step 5 : number rules
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
    
        // step 6 : remove continuous punctuation
        let continuous_punctuation_regex_pattern = format!(r"([{}]{{2,}})(\s|\z)", all_punctuations.clone());
        let continuous_punctuation_regex = Regex::new(&continuous_punctuation_regex_pattern).unwrap();
        filtered_string = continuous_punctuation_regex.replace_all(&filtered_string, " ").to_string();

        // step 7 : remove numbered references
        let numbered_reference_regex_pattern = format!(
            r"([^\d\s])(\.|∯)((\[(\d{{1,3}},?\s?-?\s?)?\b\d{{1,3}}\])+|((\d{{1,3}}\s?){{0,3}}\d{{1,3}}))( )([{}])",
            config.alphabets.clone()
        );
        let numbered_reference_regex = Regex::new(&numbered_reference_regex_pattern).unwrap();
        filtered_string = numbered_reference_regex.replace_all(&filtered_string, " ").to_string();

        // step 8 : mask the website domain
        let domain_regex = SecondRegex::new(r"\b(?:www\.)?([a-zA-Z0-9-]+\.[a-zA-Z]{2,})(?:\.[a-zA-Z]{2,})?\b").unwrap();
        let mut masked_string = domain_regex.replace_all(&filtered_string, |caps: &regex::Captures| {
            let domain = caps.get(0).unwrap().as_str();
            domain.replace(".", "&^&")
        }).to_string();

        // step 9 : remove email, geo-location, and file format
        let email_regex = Regex::new(r"(\w+)(\u{0040})(\w+)(\u{002E})(\w+)").unwrap();
        let geo_location_rule = Regex::new(r"([a-zA-Z]°)\u{002E}(\s*\d+)").unwrap();
        let file_format_rule = Regex::new(r"(\s)\u{002E}((jpe?g|png|gif|tiff?|pdf|ps|docx?|xlsx?|svg|bmp|tga|exif|odt|html?|txt|rtf|bat|sxw|xml|zip|exe|msi|blend|wmv|mp[34]|pptx?|flac|rb|cpp|cs|js)\s)").unwrap();
        masked_string = email_regex.replace_all(&masked_string, " ").to_string();
        masked_string = geo_location_rule.replace_all(&masked_string, " ").to_string();
        masked_string = file_format_rule.replace_all(&masked_string, " ").to_string();

        // step 10 : remove continuous extra periods
        let single_new_line_rule = Regex::new(r"\n").unwrap();
        let three_space_rule = Regex::new(r"(\s\.){3}\s").unwrap();
        let other_three_period_rule = Regex::new(r"\.\.\.").unwrap();
        let three_space_rule_japanese = Regex::new(r"(\s。){3}\s").unwrap();
        let other_three_period_rule_japanese = Regex::new(r"。。。").unwrap();
        let three_space_rule_chinese = Regex::new(r"(\s\u{FF0C}){3}\s").unwrap();
        let other_three_period_rule_chinese = Regex::new(r"\u{FF0C}\u{FF0C}\u{FF0C}").unwrap();
        masked_string = single_new_line_rule.replace_all(&masked_string, " ").to_string();
        masked_string = three_space_rule.replace_all(&masked_string, " ").to_string();
        masked_string = other_three_period_rule.replace_all(&masked_string, " ").to_string();
        masked_string = three_space_rule_japanese.replace_all(&masked_string, " ").to_string();
        masked_string = other_three_period_rule_japanese.replace_all(&masked_string, " ").to_string();
        masked_string = three_space_rule_chinese.replace_all(&masked_string, " ").to_string();
        masked_string = other_three_period_rule_chinese.replace_all(&masked_string, " ").to_string();

        // step 11 : remove quotations
        let between_double_quotes_regex = Regex::new(r#""(?>[^"\\]+|\\{2}|\\.)*""#).unwrap();
        let between_quote_arrow_regex = Regex::new(r"«(?>[^»\\]+|\\{2}|\\.)*»").unwrap();
        let between_quote_slanted_regex = Regex::new(r"“(?>[^”\\]+|\\{2}|\\.)*”").unwrap();
        let between_square_brackets_regex = Regex::new(r"\[(?>[^\]\\]+|\\{2}|\\.)*\]").unwrap();
        let between_parens_regex = Regex::new(r"\((?>[^\(\)\\]+|\\{2}|\\.)*\)").unwrap();
        let between_japanese_single_bracket_regex = Regex::new(r"「(?>[^」\\]+|\\{2}|\\.)*」").unwrap();
        let between_japanese_double_bracket_regex = Regex::new(r"『(?>[^』\\]+|\\{2}|\\.)*』").unwrap();
        let between_single_low_nines_regex = Regex::new(r"‚(?>[^’\\]+|\\{2}|\\.)*’").unwrap();
        let between_double_low_nines_regex = Regex::new(r"„(?>[^”\\]+|\\{2}|\\.)*”").unwrap();
        let between_full_width_double_quotes_regex = Regex::new(r"“(?>[^”\\]+|\\{2}|\\.)*”").unwrap();
        let between_full_width_single_quotes_regex = Regex::new(r"‘(?>[^’\\]+|\\{2}|\\.)*’").unwrap();
        let word_with_leading_apostrophe_regex_pattern = format!(r"(?<=\s)'(?:[^']|'[{}])*'\S", config.alphabets.clone());
        let word_with_leading_apostrophe = Regex::new(&word_with_leading_apostrophe_regex_pattern).unwrap();
        let between_em_dashes_regex = Regex::new(r"\-\-(?>[^\-\-])*\-\-").unwrap();
        masked_string = between_double_quotes_regex.replace_all(&masked_string, " ").to_string();
        masked_string = between_quote_arrow_regex.replace_all(&masked_string, " ").to_string();
        masked_string = between_quote_slanted_regex.replace_all(&masked_string, " ").to_string();
        masked_string = between_square_brackets_regex.replace_all(&masked_string, " ").to_string();
        masked_string = between_parens_regex.replace_all(&masked_string, " ").to_string();
        masked_string = between_japanese_single_bracket_regex.replace_all(&masked_string, " ").to_string();
        masked_string = between_japanese_double_bracket_regex.replace_all(&masked_string, " ").to_string();
        masked_string = between_single_low_nines_regex.replace_all(&masked_string, " ").to_string();
        masked_string = between_double_low_nines_regex.replace_all(&masked_string, " ").to_string();
        masked_string = between_full_width_double_quotes_regex.replace_all(&masked_string, " ").to_string();
        masked_string = between_full_width_single_quotes_regex.replace_all(&masked_string, " ").to_string();
        masked_string = word_with_leading_apostrophe.replace_all(&masked_string, " ").to_string();
        masked_string = between_em_dashes_regex.replace_all(&masked_string, " ").to_string();

        // step 12 : remove miscellaneous
        let three_consecutive_rule_regex_pattern = format!(r"\.\.\.(?=\s+[{}])", config.alphabets.clone());
        let three_consecutive_rule = Regex::new(&three_consecutive_rule_regex_pattern).unwrap();
        let four_consecutive_rule_regex_pattern = format!(r"\.\.\.\.(?=\s+[{}])", config.alphabets.clone());
        let four_consecutive_rule = Regex::new(&four_consecutive_rule_regex_pattern).unwrap();
        let four_space_rule_regex_pattern = format!(r"(?<=[{}])(\.\s){{3}}\.(\z|$|\n)", config.alphabets.clone());
        let four_space_rule = Regex::new(&four_space_rule_regex_pattern).unwrap();
        let between_single_quotes_regex_pattern = format!(r"(?<=\s)'(?:[^']|'[{}])*'", config.alphabets.clone());
        let between_single_quotes_regex = Regex::new(&between_single_quotes_regex_pattern).unwrap();
        let between_single_quotes_slanted_regex_pattern = format!(r"(?<=\s)‘(?:[^’]|’[{}])*’", config.alphabets.clone());
        let between_single_quote_slanted_regex = Regex::new(&between_single_quotes_slanted_regex_pattern).unwrap();
        let roman_numerals_regex = Regex::new(r"(?<=\S)\b((?=[mdclxvi])m*(c[md]|d?c*)(x[cl]|l?x*)(i[xv]|v?i*))\b(?=\s|$)").unwrap();
        masked_string = three_consecutive_rule.replace_all(&masked_string, " ").to_string();
        masked_string = four_consecutive_rule.replace_all(&masked_string, " ").to_string();
        masked_string = four_space_rule.replace_all(&masked_string, " ").to_string();
        masked_string = between_single_quotes_regex.replace_all(&masked_string, " ").to_string();
        masked_string = between_single_quote_slanted_regex.replace_all(&masked_string, " ").to_string();
        masked_string = roman_numerals_regex.replace_all(&masked_string, " ").to_string();

        // step 13 : remove extra white space
        let extra_white_space_rule = Regex::new(r"\s{1,}").unwrap();
        masked_string = extra_white_space_rule.replace_all(&masked_string, " ").to_string();

        // step 14 : mask exclamation words
        let exclamation_word_masking_map: HashMap<&str, &str> = vec![
            ("!Xũ", "&ᓴ&Xũ"), ("!Kung", "&ᓴ&Kung"), ("ǃʼOǃKung", "&ᓴ&ʼO&ᓴ&Kung"), ("!Xuun", "&ᓴ&Xuun"), ("!Kung-Ekoka", "&ᓴ&Kung&ᓴ&Ekoka"), ("ǃHu", "&ᓴ&Hu"), ("ǃKhung", "&ᓴ&Khung"), ("ǃKu", "&ᓴ&Ku"), ("ǃung", "&ᓴ&ung"), ("ǃXo", "&ᓴ&Xo"), ("ǃXû", "&ᓴ&Xû"), ("ǃXung", "&ᓴ&Xung"), ("ǃXũ", "&ᓴ&Xũ"), ("!Xun", "&ᓴ&Xun"), ("Yahoo!", "Yahoo&ᓴ&"), ("Y!J", "Y&ᓴ&J"), ("Yum!", "Yum&ᓴ&"),
        ].into_iter().collect();
        for (key, value) in exclamation_word_masking_map {
            let key_regex = Regex::new(&format!(r"\b{}\b", key)).unwrap();
            masked_string = key_regex.replace_all(&masked_string, value).to_string();
        }

        // step 15 : apply non boundary exclamation mark rules
        let exclamation_mark_before_comma_mid_sentence_regex_pattern = format!(
            r"({}|!)(?=\,\s[{}])",
            config.exclamation_mark.clone(),
            config.alphabets.clone()
        );
        let exclamation_mark_before_comma_mid_sentence_regex = Regex::new(&exclamation_mark_before_comma_mid_sentence_regex_pattern).unwrap();
        masked_string = exclamation_mark_before_comma_mid_sentence_regex.replace_all(&masked_string, "&ᓴ&").to_string();

        // step 16 : mask question mark in quotation
        let question_mark_in_quotation_regex_pattern = format!(
            r#"\{}(?=(\'|\"|[{}]))"#,
            config.question_mark.clone(),
            config.alphabets.clone(),
        );
        let question_mark_in_quotation_regex = Regex::new(&question_mark_in_quotation_regex_pattern).unwrap();
        masked_string = question_mark_in_quotation_regex.replace_all(&masked_string, "&ᓷ&").to_string();

        // step 17 : sentence segmentation and unmask
        let mut sentence_end_punctuation = vec![
            config.period.as_str(), config.question_mark.as_str(), config.exclamation_mark.as_str(),
        ];
        for punctuation in &config.other_punctuations {
            sentence_end_punctuation.push(punctuation.as_str());
        }
        let mut segmented_sentence_candidates: Vec<String> = vec![];
        let mut sentence = String::new();

        for ch in masked_string.chars() {
            if sentence_end_punctuation.contains(&ch.to_string().as_str()) {
                let mut full_sentence_candidate = sentence.trim().to_string();
                full_sentence_candidate.push(ch);
                full_sentence_candidate = full_sentence_candidate.replace("&ᓷ&", config.question_mark.clone().as_str());
                full_sentence_candidate = full_sentence_candidate.replace("&ᓴ&", config.exclamation_mark.clone().as_str());
                full_sentence_candidate = full_sentence_candidate.replace("&^&", ".");
                full_sentence_candidate = full_sentence_candidate.trim().to_string();
                let extra_white_space_rule = Regex::new(r"\s{1,}").unwrap();
                full_sentence_candidate = extra_white_space_rule.replace_all(&full_sentence_candidate, " ").to_string();
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
            full_sentence_candidate.push(config.period.clone().as_str().chars().next().unwrap());
            full_sentence_candidate = full_sentence_candidate.replace("&ᓷ&", config.question_mark.clone().as_str());
            full_sentence_candidate = full_sentence_candidate.replace("&ᓴ&", config.exclamation_mark.clone().as_str());
            full_sentence_candidate = full_sentence_candidate.replace("&^&", ".");
            full_sentence_candidate = full_sentence_candidate.trim().to_string();
            if full_sentence_candidate.len() > 2 {
                segmented_sentence_candidates.push(full_sentence_candidate);
            }
        }

        let mut final_segmented_sentences: Vec<String> = vec![];

        if !config.have_capital_letter {
            final_segmented_sentences = segmented_sentence_candidates;
        } else {
            // if the first letter is lowercase, merge it with the previous sentence
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
        }

	for (k, v) in abbreviations {
	    unsafe{
		let ptr_k = k as *const _;
		let ptr_v = v as *const _;
		core::ptr::drop_in_place(&ptr_k as *const _ as *mut Box<[&str; 3]>);
		core::ptr::drop_in_place(&ptr_v as *const _ as *mut Box<[&str; 3]>);		    
	    }
	}
	
        final_segmented_sentences
    }

    pub fn afrikaans(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn albanian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÇçËë".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn amharic(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{1200}-\u{137F}".to_string(),
            have_capital_letter: false,
            period: "\u{1362}".to_string(),
            question_mark: "\u{1367}".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn arabic(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0600}-\u{06FF}\u{08A0}-\u{08FF}\u{0870}-\u{089F}\u{FB50}-\u{FDFF}".to_string(),
            have_capital_letter: false,
            period: ".".to_string(),
            question_mark: "\u{061F}".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["?".to_string(), "\u{06D4}".to_string()],
        };

        process(text, config)
    }

    pub fn armenian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0531}-\u{058F}".to_string(),
            have_capital_letter: true,
            period: "\u{0589}".to_string(),
            question_mark: "\u{055E}".to_string(),
            exclamation_mark: "\u{055C}".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn assamese(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0980}-\u{09FF}".to_string(),
            have_capital_letter: false,
            period: "\u{002E}".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![".".to_string(), "\u{0964}".to_string(), "\u{0965}".to_string()],
        };

        process(text, config)
    }

    pub fn azerbaijani(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÇŞĞÖÜİƏçşğıöüə".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn balinese(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{1B00}-\u{1B7F}".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{1B4F}".to_string(), "\u{1B5A}".to_string(), "\u{1B7D}".to_string(), "\u{1B7E}".to_string()],
        };

        process(text, config)
    }

    pub fn basque(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÑÇñç".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn belarusian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0400}-\u{04FF}\u{0500}-\u{052F}\u{2DE0}-\u{2DFF}\u{A640}-\u{A69F}\u{1C80}-\u{1C8F}\u{1E030}-\u{1E08F}\u{1D2B}\u{1D78}\u{FE2E}\u{FE2F}".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn bengali(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0980}-\u{09FF}".to_string(),
            have_capital_letter: false,
            period: "\u{0964}".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![".".to_string(), "\u{0965}".to_string()],
        };

        process(text, config)
    }

    pub fn bosnian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZČĆDžĐŠŽčćdžđšž".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn bulgarian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0400}-\u{04FF}\u{0500}-\u{052F}\u{2DE0}-\u{2DFF}\u{A640}-\u{A69F}\u{1C80}-\u{1C8F}\u{1E030}-\u{1E08F}\u{1D2B}\u{1D78}\u{FE2E}\u{FE2F}".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn burmese(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{1000}-\u{109F}".to_string(),
            have_capital_letter: false,
            period: "\u{104B}".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![".".to_string()],
        };

        process(text, config)
    }

    pub fn catalan(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÀÁÂÈÉÊÌÍÒÓÔÙÚÜÇàáâèéêìíòóôùúüç".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn cebuano(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÑñ".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn chechen(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0400}-\u{04FF}\u{0500}-\u{052F}\u{2DE0}-\u{2DFF}\u{A640}-\u{A69F}\u{1C80}-\u{1C8F}\u{1E030}-\u{1E08F}\u{1D2B}\u{1D78}\u{FE2E}\u{FE2F}".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn chinese(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{4E00}-\u{9FFF}".to_string(),
            have_capital_letter: false,
            period: "。".to_string(),
            question_mark: "？".to_string(),
            exclamation_mark: "！".to_string(),
            other_punctuations: vec!["!".to_string(), "?".to_string()],
        };

        process(text, config)
    }

    pub fn creole(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÈèÒò".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    // basically same as serbian
    pub fn croatian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZČĆŽŠĐčćžšđa-zA-Z\u{0400}-\u{04FF}\u{0500}-\u{052F}\u{2DE0}-\u{2DFF}\u{A640}-\u{A69F}\u{1C80}-\u{1C8F}\u{1E030}-\u{1E08F}\u{1D2B}\u{1D78}\u{FE2E}\u{FE2F}".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn czech(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZĚŠČŘŽÝÁÍÉÚŮÓěščřžýáíéúůó".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn danish(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZØÆÅøæå".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn dinka(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÄäËëƐ̈ɛ̈ƔɣÏïŊŋÖöƆɔƆ̈ɔ̈Üü".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn dutch(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÄËÏÖÜÇÀÈÌÒÙäëïöüçàèìòù".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn english(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn esperanto(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZĈĉĜĝĤĥĴĵŜŝŬŭ".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn estonian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZŠŽÕÄÖÜšžõäöü".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn finnish(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÄÖÅäöå".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn french(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÀÂÆÇÉÈÊËÎÏÔŒÙÛÜŸàâæçéèêëîïôœùûüÿ".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn galician(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÑñ".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn ganda(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÀÈÙÁÉÍÓÚàèùáéíóú".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn georgian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{10A0}-\u{10FF}".to_string(),
            have_capital_letter: false,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn german(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÄÖÜẞäöüß".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn greek(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0370}-\u{03FF}\u{1F00}-\u{1FFF}\u{1D00}-\u{1D7F}\u{1D80}-\u{1DBF}\u{2100}-\u{214F}".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn gujarati(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0A80}-\u{0AFF}".to_string(),
            have_capital_letter: false,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn hausa(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZƁɓƊɗƘƙƳƴ".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn hebrew(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0590}-\u{05FF}".to_string(),
            have_capital_letter: false,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn hindi(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0900}-\u{097F}\u{A8E0}-\u{A8FF}\u{11B00}-\u{11B5F}\u{1CD0}-\u{1CFF}".to_string(),
            have_capital_letter: false,
            period: "\u{0964}".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![".".to_string(), "\u{0965}".to_string()],
        };

        process(text, config)
    }

    pub fn hungarian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÁÉÍÓÖŐÚÜáéíóöőúü".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn icelandic(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÁáÉéÍíÓóÚúÝýÞþÆæÖöÐð".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn igbo(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZỊṄỌỤịṅọụ".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn ido(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn indonesian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "A-Za-zÁáÉéÍíÓóÚúŃńÇçĐđ".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn interlingua(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn irish(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÁáÉéÍíÓóÚúÇç".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn italian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÀÈÉÌÒÙàèéìòù".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn japanese(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{3040}-\u{309F}\u{30A0}-\u{30FF}\u{4000}-\u{9FFF}".to_string(),
            have_capital_letter: false,
            period: "。".to_string(),
            question_mark: "？".to_string(),
            exclamation_mark: "！".to_string(),
            other_punctuations: vec![".".to_string()],
        };

        process(text, config)
    }

    pub fn javanese(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÅåÉéĚěÓóÚú\u{A980}-\u{A9DF}".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn kannada(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0C80}-\u{0CFF}".to_string(),
            have_capital_letter: false,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn kashmiri(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0600}-\u{06FF}\u{08A0}-\u{08FF}\u{0870}-\u{089F}\u{FB50}-\u{FDFF}\u{0900}-\u{097F}\u{A8E0}-\u{A8FF}\u{11B00}-\u{11B5F}\u{1CD0}-\u{1CFF}\u{11180}-\u{111DF}".to_string(),
            have_capital_letter: false,
            period: "\u{111C5}".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![".".to_string(), "\u{06D4}".to_string(), "\u{0964}".to_string(), "\u{0965}".to_string(), "\u{111C6}".to_string(), "\u{111C8}".to_string()],
        };

        process(text, config)
    }

    pub fn kazakh(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "A-Za-zÁáÀàÂâÄäÇçEéÍíOóŞşUú\u{0400}-\u{04FF}\u{0500}-\u{052F}\u{2DE0}-\u{2DFF}\u{A640}-\u{A69F}\u{1C80}-\u{1C8F}\u{1E030}-\u{1E08F}\u{1D2B}\u{1D78}\u{FE2E}\u{FE2F}".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn khmer(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "A-Za-z\u{1780}-\u{17FF}".to_string(),
            have_capital_letter: false,
            period: "\u{17D4}".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "\u{17D5}".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn korean(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{AC00}-\u{D7A3}\u{4000}-\u{9FFF}".to_string(),
            have_capital_letter: false,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn kurdish(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÇÊÎŞÛçêîşû".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn kyrgyz(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0400}-\u{04FF}\u{0500}-\u{052F}\u{2DE0}-\u{2DFF}\u{A640}-\u{A69F}\u{1C80}-\u{1C8F}\u{1E030}-\u{1E08F}\u{1D2B}\u{1D78}\u{FE2E}\u{FE2F}".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn lao(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0E80}-\u{0EFF}".to_string(),
            have_capital_letter: false,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn latin(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn latvian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZĀČĒĢĪĶĻŅŌŖŠŪŽāčēģīķļņōŗšūž".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn lithuanian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZĄČĘĖĮŠŲŪŽąčęėįšųūž".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn macedonian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0400}-\u{04FF}\u{0500}-\u{052F}\u{2DE0}-\u{2DFF}\u{A640}-\u{A69F}\u{1C80}-\u{1C8F}\u{1E030}-\u{1E08F}\u{1D2B}\u{1D78}\u{FE2E}\u{FE2F}".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn malagasy(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    // basically same as indonesian
    pub fn malay(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÁáÉéÍíÓóÚúŃńÇçĐđ".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn malayalam(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0D00}-\u{0D7F}".to_string(),
            have_capital_letter: false,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn maltese(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZĊċĠġĦħŻż".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn manipuri(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{ABC0}-\u{ABFF}".to_string(),
            have_capital_letter: true,
            period: "\u{ABEB}".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![".".to_string()],
        };

        process(text, config)
    }

    pub fn maori(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZĀāĒēĪīŌōŪū".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn marathi(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0900}-\u{097F}\u{A8E0}-\u{A8FF}\u{11B00}-\u{11B5F}\u{1CD0}-\u{1CFF}".to_string(),
            have_capital_letter: false,
            period: "\u{0964}".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![".".to_string(), "\u{0965}".to_string()],
        };

        process(text, config)
    }

    pub fn mongolian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0400}-\u{04FF}\u{0500}-\u{052F}\u{2DE0}-\u{2DFF}\u{A640}-\u{A69F}\u{1C80}-\u{1C8F}\u{1E030}-\u{1E08F}\u{1D2B}\u{1D78}\u{FE2E}\u{FE2F}".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn nepali(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0900}-\u{097F}\u{A8E0}-\u{A8FF}\u{11B00}-\u{11B5F}\u{1CD0}-\u{1CFF}".to_string(),
            have_capital_letter: false,
            period: "\u{0964}".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![".".to_string(), "\u{0965}".to_string()],
        };

        process(text, config)
    }

    pub fn norwegian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÆØÅæøå".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn oriya(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0B00}-\u{0B7F}".to_string(),
            have_capital_letter: false,
            period: "\u{0964}".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![".".to_string(), "\u{0965}".to_string()],
        };

        process(text, config)
    }

    pub fn ossetian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0400}-\u{04FF}\u{0500}-\u{052F}\u{2DE0}-\u{2DFF}\u{A640}-\u{A69F}\u{1C80}-\u{1C8F}\u{1E030}-\u{1E08F}\u{1D2B}\u{1D78}\u{FE2E}\u{FE2F}".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn pashto(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0600}-\u{06FF}\u{08A0}-\u{08FF}\u{0870}-\u{089F}\u{FB50}-\u{FDFF}".to_string(),
            have_capital_letter: false,
            period: "\u{06D4}".to_string(),
            question_mark: "\u{061F}".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![".".to_string()],
        };

        process(text, config)
    }

    pub fn persian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0600}-\u{06FF}\u{08A0}-\u{08FF}\u{0870}-\u{089F}\u{FB50}-\u{FDFF}".to_string(),
            have_capital_letter: false,
            period: ".".to_string(),
            question_mark: "\u{061F}".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{06D4}".to_string()],
        };

        process(text, config)
    }

    pub fn polish(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZĄĆĘŁŃÓŚŹŻąćęłńóśźż".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn portuguese(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÀÁÂÃÄÇÉÊËÍÎÏÓÔÕÖÚÛÜàáâãäçéêëíîïóôõöúûü".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn punjabi_eastern(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0A00}-\u{0A7F}".to_string(),
            have_capital_letter: false,
            period: "\u{0964}".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string(), "\u{0965}".to_string(), ".".to_string()],
        };

        process(text, config)
    }

    pub fn punjabi_western(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0600}-\u{06FF}\u{08A0}-\u{08FF}\u{0870}-\u{089F}\u{FB50}-\u{FDFF}".to_string(),
            have_capital_letter: false,
            period: ".".to_string(),
            question_mark: "\u{061F}".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["?".to_string(), "\u{06D4}".to_string()],
        };

        process(text, config)
    }

    pub fn romanian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZĂÂÎȘȚăâîșț".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn russian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0400}-\u{04FF}\u{0500}-\u{052F}\u{2DE0}-\u{2DFF}\u{A640}-\u{A69F}\u{1C80}-\u{1C8F}\u{1E030}-\u{1E08F}\u{1D2B}\u{1D78}\u{FE2E}\u{FE2F}".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn sanskrit(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0900}-\u{097F}\u{A8E0}-\u{A8FF}\u{11B00}-\u{11B5F}\u{1CD0}-\u{1CFF}".to_string(),
            have_capital_letter: false,
            period: "\u{0964}".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![".".to_string(), "\u{0965}".to_string()],
        };

        process(text, config)
    }

    pub fn santali(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{1C50}-\u{1C7F}".to_string(),
            have_capital_letter: false,
            period: "\u{1C7E}".to_string(),
            question_mark: "\u{1C76}".to_string(),
            exclamation_mark: "\u{1C7F}".to_string(),
            other_punctuations: vec!["\u{0964}".to_string(), "\u{0965}".to_string(), ".".to_string()],
        };

        process(text, config)
    }

    pub fn scottish(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn serbian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZščćđžŠČĆĐŽ\u{0400}-\u{04FF}\u{0500}-\u{052F}\u{2DE0}-\u{2DFF}\u{A640}-\u{A69F}\u{1C80}-\u{1C8F}\u{1E030}-\u{1E08F}\u{1D2B}\u{1D78}\u{FE2E}\u{FE2F}".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn shona(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZáÁâÂàÀéÉèÈíÍìÌóÓòÒúÚùÙñÑ".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn sindhi(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0600}-\u{06FF}\u{08A0}-\u{08FF}\u{0870}-\u{089F}\u{FB50}-\u{FDFF}\u{0900}-\u{097F}\u{A8E0}-\u{A8FF}\u{11B00}-\u{11B5F}\u{1CD0}-\u{1CFF}".to_string(),
            have_capital_letter: false,
            period: "\u{0964}".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![".".to_string(), "\u{0965}".to_string(), "\u{06D4}".to_string()],
        };

        process(text, config)
    }

    pub fn sinhala(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0D80}-\u{0DFF}".to_string(),
            have_capital_letter: false,
            period: "\u{0964}".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{0DF4}".to_string(), "\u{0965}".to_string(), ".".to_string()],
        };

        process(text, config)
    }

    pub fn slovak(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÁÄČĎÉĽÍŇÓÔÚÝáäčďéľíňóôúý".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn slovenian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZČŠŽčšž".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn somali(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÀàÁáÂâÆæÈèÉéÊêËëÍíÎîÒòÓóÔôÙùÚúÛû".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn sotho(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÀàÁáÈèÉéÊêÍíÒòÓóÔôÙùÚú".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn spanish(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÁÉÍÓÚÑáéíóúñ".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn sundanese(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0600}-\u{06FF}\u{08A0}-\u{08FF}\u{0870}-\u{089F}\u{FB50}-\u{FDFF}".to_string(),
            have_capital_letter: false,
            period: ".".to_string(),
            question_mark: "\u{061F}".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["?".to_string(), "\u{06D4}".to_string()],
        };

        process(text, config)
    }

    pub fn swahili(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn swedish(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÅÄÖåäö".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn tagalog(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn tamil(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0B80}-\u{0BFF}".to_string(),
            have_capital_letter: false,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn tatar(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0400}-\u{04FF}\u{0500}-\u{052F}\u{2DE0}-\u{2DFF}\u{A640}-\u{A69F}\u{1C80}-\u{1C8F}\u{1E030}-\u{1E08F}\u{1D2B}\u{1D78}\u{FE2E}\u{FE2F}".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn telugu(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0C00}-\u{0C7F}".to_string(),
            have_capital_letter: false,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{061F}".to_string()],
        };

        process(text, config)
    }

    pub fn tibetan(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0F00}-\u{0FFF}".to_string(),
            have_capital_letter: false,
            period: "\u{0F0D}".to_string(),
            question_mark: "\u{2048}".to_string(),
            exclamation_mark: "\u{0FC8}".to_string(),
            other_punctuations: vec!["\u{0F0E}".to_string(), "\u{0F12}".to_string(), "\u{0F00}".to_string(), "\u{0F01}".to_string(), "\u{0F09}".to_string(), "\u{0F0A}".to_string(), ".".to_string()],
        };

        process(text, config)
    }

    pub fn tsonga(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn tswana(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn turkish(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZĞğİıÇçŞşÖöÜü".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }
    
    pub fn turkmen(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÄäÇçĞğÑñÖöŞşÜü".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn ukrainian(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0400}-\u{04FF}\u{0500}-\u{052F}\u{2DE0}-\u{2DFF}\u{A640}-\u{A69F}\u{1C80}-\u{1C8F}\u{1E030}-\u{1E08F}\u{1D2B}\u{1D78}\u{FE2E}\u{FE2F}".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn urdu(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0600}-\u{06FF}\u{08A0}-\u{08FF}\u{0870}-\u{089F}\u{FB50}-\u{FDFF}".to_string(),
            have_capital_letter: false,
            period: ".".to_string(),
            question_mark: "\u{061F}".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{06D4}".to_string()],
        };

        process(text, config)
    }

    pub fn uyghur(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0600}-\u{06FF}\u{08A0}-\u{08FF}\u{0870}-\u{089F}\u{FB50}-\u{FDFF}".to_string(),
            have_capital_letter: false,
            period: ".".to_string(),
            question_mark: "\u{061F}".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec!["\u{06D4}".to_string()],
        };

        process(text, config)
    }

    pub fn uzbek(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZŌŞÇḠōşçḡ\u{0400}-\u{04FF}\u{0500}-\u{052F}\u{2DE0}-\u{2DFF}\u{A640}-\u{A69F}\u{1C80}-\u{1C8F}\u{1E030}-\u{1E08F}\u{1D2B}\u{1D78}\u{FE2E}\u{FE2F}".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn vietnamese(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZÀÁÂÃÈÉÊÌÍÒÓÔÕÙÚĂĐĨŨƠƯàáâãèéêìíòóôõùúăđĩũơưỲÝỴỶỸỳýỵỷỹ".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn volapuk(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZäöüÄÖÜ".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn welsh(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn xhosa(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn yiddish(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z\u{0590}-\u{05FF}".to_string(),
            have_capital_letter: false,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }   

    pub fn yoruba(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-ZẸỌṢẹọṣ".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }

    pub fn zulu(text: &str) -> Vec<String> {
        let config = LanguageConfig {
            alphabets: "a-zA-Z".to_string(),
            have_capital_letter: true,
            period: ".".to_string(),
            question_mark: "?".to_string(),
            exclamation_mark: "!".to_string(),
            other_punctuations: vec![],
        };

        process(text, config)
    }
}
