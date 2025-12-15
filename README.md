# Sentence Segmentation

This is a regex rule-based sentence segmenter written in pure rust. This crate is inspired by the Ruby Pragmatic Segmenter by diasks2 ([GitHub Repo](https://github.com/diasks2/pragmatic_segmenter)).

## Features

- **Multilingual Support**: The crate supports sentence segmentation for 147 languages (see list below).
```
processor::abazanian
processor::abkhazian
processor::afrikaans
processor::albanian
processor::amharic
processor::arabic
processor::armenian
processor::assamese
processor::azerbaijani
processor::balinese
processor::balochi
processor::bambara
processor::basque
processor::beja
processor::belarusian
processor::bengali
processor::brahui
processor::bhojpuri
processor::bosnian
processor::bulgarian
processor::burmese
processor::buryat
processor::catalan
processor::cebuano
processor::chechen
processor::chinese
processor::corsican
processor::creole
processor::croatian
processor::czech
processor::danish
processor::dargwa
processor::dinka
processor::dutch
processor::english
processor::erzya
processor::esperanto
processor::estonian
processor::finnish
processor::french
processor::frisian
processor::galician
processor::ganda
processor::gedeo
processor::georgian
processor::german
processor::greek
processor::guarani
processor::gujarati
processor::hausa
processor::hebrew
processor::hiligaynon
processor::hindi
processor::hungarian
processor::icelandic
processor::igbo
processor::ido
processor::indonesian
processor::interlingua
processor::irish
processor::italian
processor::japanese
processor::javanese
processor::kabyle
processor::kangri
processor::kannada
processor::kashmiri
processor::kazakh
processor::khmer
processor::khoekhoe
processor::kiga
processor::korean
processor::kurdish
processor::kyrgyz
processor::lao
processor::latin
processor::latgalian
processor::latvian
processor::lithuanian
processor::macedonian
processor::magahi
processor::malagasy
processor::malay
processor::malayalam
processor::maltese
processor::mandeali
processor::manipuri
processor::maori
processor::marathi
processor::mongolian
processor::nepali
processor::nkore
processor::norwegian
processor::oriya
processor::ossetian
processor::papiamento
processor::pashto
processor::persian
processor::polish
processor::portuguese
processor::punjabi_eastern
processor::punjabi_western
processor::quechuan
processor::romanian
processor::russian
processor::sanskrit
processor::santali
processor::scottish
processor::serbian
processor::shona
processor::sindhi
processor::sinhala
processor::slovak
processor::slovenian
processor::somali
processor::sotho
processor::spanish
processor::sundanese
processor::swahili
processor::swedish
processor::tagalog
processor::tamil
processor::tatar
processor::telugu
processor::tetum
processor::thai // See below
processor::tibetan
processor::tigrinya
processor::tsonga
processor::tswana
processor::turkish
processor::turkmen
processor::ukrainian
processor::urdu
processor::uyghur
processor::uzbek
processor::vietnamese
processor::volapuk
processor::welsh
processor::wolof
processor::xhosa
processor::yakut
processor::yiddish
processor::yoruba
processor::zaza
processor::zulu
```
- **Modular Segmentation**: Each language has a dedicated segmentation function, making it simple to use language-specific rules.
- **Preprocessing**: Through regex, the input text will also be preprocessed and denoised. The result will be a vector of cleaned strings which are ready for the normalization that you planned. 

## Installation

To use this crate, add it to your `Cargo.toml` dependencies:

```
[dependencies]
sentence_segmentation = "1.3.0"
```

## Usage

```
use sentence_segmentation::processor;

fn main() {
    let text = "이 크레이트는 자연어 처리를 위해서 만들어진 크레이트입니다. 정규표현식 기반의 간단한 알고리즘을 통해 문장 단위로 분리하기에 매우 빠른 속도로 문장을 분리할 수 있습니다. 이 크레이트는 기본적인 전처리도 같이 수행하므로 문장 분리가 목적이 아니더라도 전처리를 위해서 사용할 수도 있습니다.";
    let sentences = processor::korean(text);

    for sentence in sentences {
        println!("{}", sentence);
    }
    // 이 크레이트는 자연어 처리를 위해서 만들어진 크레이트입니다.
    // 정규표현식 기반의 간단한 알고리즘을 통해 문장 단위로 분리하기에 매우 빠른 속도로 문장을 분리할 수 있습니다.
    // 이 크레이트는 기본적인 전처리도 같이 수행하므로 문장 분리가 목적이 아니더라도 전처리를 위해서 사용할 수도 있습니다.

}
```

## For Thai Users

Thai usually does not use punctuations to end sentences, so I applied a simple cnn model to separate sentences, in order not to unnecessarily install the related crates such as burn, thai sentence segmenting function is in separate flag. You can activate the function by specifying feature in cargo.toml,

```
[dependencies]
sentence_segmentation = {version = "1.3.0", features = ["thai"]}
```

## License

This project is licensed under the MIT License. See the LICENSE file for details.
