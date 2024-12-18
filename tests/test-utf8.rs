#![cfg(feature = "utf8")]
use morse_codec::{
    decoder::{Decoder, Precision}, CharacterSet, MorseCodeSet, MorseSignal::{Long as L, Short as S}, MORSE_DEFAULT_CHAR
};

#[test]
fn utf8_decoding() {
    const MESSAGE_MAX_LENGTH: usize = 256;

    #[cfg(feature = "utf8")]
    let character_set: CharacterSet = &[
        ' ',
        'Α', 'Β', 'Γ', 'Δ', 'Ε', 'Ζ', 'Η', 'Θ', 'Ι', 'Κ', 'Λ', 'Μ', 'Ν', 'Ξ', 'Ο', 'Π', 'Ρ', 'Σ', 'Τ',
        'Υ', 'Φ', 'Χ', 'Ψ', 'Ω',
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
        ',', '?', ':', '-', '"', '(', '=', 'X', '.', ';', '/', '\'', '_', ')', '+', '@',
    ];

    #[cfg(feature = "utf8")]
    let morse_code_set: MorseCodeSet =
        &[
            //
            // Default char is empty character
            MORSE_DEFAULT_CHAR, // Empty character ' '
            //
            // Letters
            [Some(S), Some(L), None, None, None, None],       // A
            [Some(L), Some(S), Some(S), Some(S), None, None], // B
            [Some(L), Some(L), Some(S), None, None, None],    // Γ
            [Some(L), Some(S), Some(S), None, None, None],    // Δ
            [Some(S), None, None, None, None, None],          // E
            [Some(L), Some(L), Some(S), Some(S), None, None], // Z
            [Some(S), Some(S), Some(S), Some(S), None, None], // H
            [Some(L), Some(S), Some(L), Some(S), None, None], // Θ
            [Some(S), Some(S), None, None, None, None],       // I
            [Some(L), Some(S), Some(L), None, None, None],    // K
            [Some(S), Some(L), Some(S), Some(S), None, None], // Λ
            [Some(L), Some(L), None, None, None, None],       // M
            [Some(L), Some(S), None, None, None, None],       // N
            [Some(L), Some(S), Some(S), Some(L), None, None], // Ξ
            [Some(L), Some(L), Some(L), None, None, None],    // O
            [Some(S), Some(L), Some(L), Some(S), None, None], // Π
            [Some(S), Some(L), Some(S), None, None, None],    // Ρ
            [Some(S), Some(S), Some(S), None, None, None],    // Σ
            [Some(L), None, None, None, None, None],          // T
            [Some(L), Some(S), Some(L), Some(L), None, None], // Y
            [Some(S), Some(S), Some(L), Some(S), None, None], // Φ
            [Some(L), Some(L), Some(L), Some(L), None, None], // X
            [Some(L), Some(L), Some(S), Some(L), None, None], // Ψ
            [Some(S), Some(L), Some(L), None, None, None],    // Ω
            //
            // Numbers
            [Some(S), Some(L), Some(L), Some(L), Some(L), None], // 1
            [Some(S), Some(S), Some(L), Some(L), Some(L), None], // 2
            [Some(S), Some(S), Some(S), Some(L), Some(L), None], // 3
            [Some(S), Some(S), Some(S), Some(S), Some(L), None], // 4
            [Some(S), Some(S), Some(S), Some(S), Some(S), None], // 5
            [Some(L), Some(S), Some(S), Some(S), Some(S), None], // 6
            [Some(L), Some(L), Some(S), Some(S), Some(S), None], // 7
            [Some(L), Some(L), Some(L), Some(S), Some(S), None], // 8
            [Some(L), Some(L), Some(L), Some(L), Some(S), None], // 9
            [Some(L), Some(L), Some(L), Some(L), Some(L), None], // 0
            //
            // Punctuation marks
            [Some(L), Some(L), Some(S), Some(S), Some(L), Some(L)], // Comma                ,
            [Some(S), Some(S), Some(L), Some(L), Some(S), Some(S)], // Question mark        ?
            [Some(L), Some(L), Some(L), Some(S), Some(S), Some(S)], // Colon                :
            [Some(L), Some(S), Some(S), Some(S), Some(S), Some(L)], // Dash                 -
            [Some(S), Some(L), Some(S), Some(S), Some(L), Some(S)], // Double quote         "
            [Some(L), Some(S), Some(L), Some(L), Some(S), None],    // Left bracket         (
            [Some(L), Some(S), Some(S), Some(S), Some(L), None],    // Equals               =
            [Some(L), Some(S), Some(S), Some(L), None, None],       // Multiplication       X
            [Some(S), Some(L), Some(S), Some(L), Some(S), Some(L)], // Full stop (period)   .
            [Some(L), Some(S), Some(L), Some(S), Some(L), Some(S)], // Semicolon            ;
            [Some(L), Some(S), Some(S), Some(L), Some(S), None],    // Slash                /
            [Some(S), Some(L), Some(L), Some(L), Some(L), Some(S)], // Apostrophe           '
            [Some(S), Some(S), Some(L), Some(L), Some(S), Some(L)], // Underscore           _
            [Some(L), Some(S), Some(L), Some(L), Some(S), Some(L)], // Right bracket        )
            [Some(S), Some(L), Some(S), Some(L), Some(S), None],    // Addition             +
            [Some(S), Some(L), Some(L), Some(S), Some(L), Some(S)], // At sign              @
        ];

    println!("TEST DECODING UTF8 CHARACTERS WITH GREEK ALPHABET:");

    println!();
    character_set.iter().for_each(|ch| print!("{} ", *ch as char));
    println!();
//Η ΟΔΎΣΣΕΙΑ ΠΟΥ ΑΠΟΤΕΛΕΊΤΑΙ ΑΠΌ 12.110 ΣΤΊΧΟΥΣ

    let mut decoder = Decoder::<MESSAGE_MAX_LENGTH>::new()
        .with_precision(Precision::Accurate)
        .with_character_set(character_set)
        .with_morse_code_set(morse_code_set)
        .build();

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(700, false);

    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(700, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(700, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(700, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(700, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(700, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    let message_str = decoder.message.as_str();

    println!("Message is {}", message_str);
}

