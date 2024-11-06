# Sentence Segmentation

This is a regex rule-based sentence segmenter written in pure rust. This crate is inspired by the Ruby Pragmatic Segmenter by diasks2 ([GitHub Repo](https://github.com/diasks2/pragmatic_segmenter)).

## Features

- **Multilingual Support**: The crate supports sentence segmentation for a wide variety of languages (see list below).
```
processor::amharic
processor::afrikaans
processor::albanian
processor::arabic
processor::armenian
processor::azerbaijani
processor::basque
processor::belarusian
processor::bengali
processor::bosnian
processor::bulgarian
processor::catalan
processor::chinese
processor::croatian
processor::czech
processor::danish
processor::dutch
processor::english
processor::esperanto
processor::estonian
processor::finnish
processor::french
processor::ganda
processor::georgian
processor::german
processor::greek
processor::gujarati
processor::hausa
processor::hebrew
processor::hindi
processor::hungarian
processor::icelandic
processor::indonesian
processor::irish
processor::italian
processor::japanese
processor::kazakh
processor::korean
processor::latin
processor::latvian
processor::lithuanian
processor::macedonian
processor::malay
processor::maori
processor::marathi
processor::mongolian
processor::norwegian
processor::persian
processor::polish
processor::portuguese
processor::punjabi_eastern
processor::punjabi_western
processor::romanian
processor::russian
processor::serbian
processor::shona
processor::slovak
processor::slovenian
processor::somali
processor::sotho
processor::spanish
processor::swahili
processor::swedish
processor::tagalog
processor::tamil
processor::telugu
processor::tsonga
processor::tswana
processor::turkish
processor::ukrainian
processor::urdu
processor::vietnamese
processor::welsh
processor::xhosa
processor::yoruba
processor::zulu
```
- **Modular Segmentation**: Each language has a dedicated segmentation function, making it simple to use language-specific rules.
- **Preprocessing**: Through regex, the input text will also be preprocessed and denoised. The result will be a vector of cleaned strings which are ready for the normalization that you planned. 

## Installation

To use this crate, add it to your `Cargo.toml` dependencies:

```
[dependencies]
sentence_segmentation = "1.0.0"
```

## Usage

```
use sentence_segmentation::processor;

fn main() {
    let text = "이 크레이트는 자연어 처리를 위해서 만들어진 크레이트입니다. 정규표현식 기반의 간단한 알고리즘을 통해 문장 단위로 분리하기에 매우 빠른 속도로 문장을 분리할 수 있습니다. 이 크레이트는 기본적인 전처리도 같이 수행하므로 문장 분리가 목적이 아니더라도 전처리를 위해서 사용할 수도 있습니다.";
    let sentences = processor::english(text);

    for sentence in sentences {
        println!("{}", sentence);
    }
}
```

## License

This project is licensed under the MIT License. See the LICENSE file for details.
