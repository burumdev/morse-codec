//! Rust library for live decoding and encoding of morse code messages.
//! Supports multiple embedded devices and operating systems by being no_std.
//!
//! You can create messages by sending individual high and low signals in milliseconds to decoder,
//! from the keyboard, mouse clicks, or a button connected to some embedded device.
//! Decoder supports three precision (difficulty) modes. Lazy (easiest), Accurate(Hardest) and
//! Farnsworth mode (somewhere inbetween)
//!
//! Use the encoder to turn your messages or characters into morse code strings or create a
//! sequence of signals to drive an external component such as an LED, step motor or speaker.
//!
//! # Features
//! * Decoder
//! * Encoder
//!
//! UTF-8 is not supported at the moment, but can be implemented behind
//! a feature flag in the future.
//!
//! The lib is no_std outside testing to make sure it will work on embedded devices
//! as well as operating systems.

// There're debug println!() statements in various parts of
// the code marked by a "// DBG" sign on top. In order to use them on a development environment
// with a proper OS and std, comment out the below attribute and uncomment the debug lines you want.

#![cfg_attr(not(test), no_std)]

// This is the array length for a sequence of morse signals or character representation of those
// signals while encoding
const MORSE_ARRAY_LENGTH: usize = 6;

/// Maximum number of characters in a mapping set of morse code to letters.
pub const CHARACTER_SET_LENGTH: usize = 53;

const LONG_SIGNAL_MULTIPLIER: u16 = 3;
const WORD_SPACE_MULTIPLIER: u16 = 7;

/// We use this character to fill message arrays so when we encounter this char
/// it actually means there's no character there.
///
/// The character # is not a part of international morse code, so it's a good candidate.
pub const FILLER_BYTE: u8 = b'#';

/// Char version of the [FILLER_BYTE] coz why not? It's mainly used while generating bytes from
/// &str slices. A [char] which is utf-8 by default in Rust, can be more than one byte, turning
/// chars into bytes if they're ascii makes the code stable.
pub const FILLER_CHAR: char = FILLER_BYTE as char;

/// If a decoding error happens, we put this character as a placeholder.
pub const DECODING_ERROR_BYTE: u8 = b'?';

/// Building block of morse characters.
///
/// This enum can be used with the decoder to directly add signals to characters.
#[derive(Clone, Debug, PartialEq)]
pub enum MorseSignal {
    Short,
    Long,
}
use MorseSignal::{Long as L, Short as S};

type MorseCodeArray = [Option<MorseSignal>; MORSE_ARRAY_LENGTH];

/// This corresponds to empty character ' ' which is the default character
pub const MORSE_DEFAULT_CHAR: MorseCodeArray = [None, None, None, None, None, None];

/// Internal representation of morse characters. It's an array of length [CHARACTER_SET_LENGTH].
///
/// Letters can be converted to these morse code arrays and vice-versa.
pub const MORSE_CODE_SET: [MorseCodeArray; CHARACTER_SET_LENGTH] = [
    //
    // Default char is empty character
    MORSE_DEFAULT_CHAR, // Empty character ' '
    //
    // Letters
    [Some(S), Some(L), None, None, None, None],       // A
    [Some(L), Some(S), Some(S), Some(S), None, None], // B
    [Some(L), Some(S), Some(L), Some(S), None, None], // C
    [Some(L), Some(S), Some(S), None, None, None],    // D
    [Some(S), None, None, None, None, None],          // E
    [Some(S), Some(S), Some(L), Some(S), None, None], // F
    [Some(L), Some(L), Some(S), None, None, None],    // G
    [Some(S), Some(S), Some(S), Some(S), None, None], // H
    [Some(S), Some(S), None, None, None, None],       // I
    [Some(S), Some(L), Some(L), Some(L), None, None], // J
    [Some(L), Some(S), Some(L), None, None, None],    // K
    [Some(S), Some(L), Some(S), Some(S), None, None], // L
    [Some(L), Some(L), None, None, None, None],       // M
    [Some(L), Some(S), None, None, None, None],       // N
    [Some(L), Some(L), Some(L), None, None, None],    // O
    [Some(S), Some(L), Some(L), Some(S), None, None], // P
    [Some(L), Some(L), Some(S), Some(L), None, None], // Q
    [Some(S), Some(L), Some(S), None, None, None],    // R
    [Some(S), Some(S), Some(S), None, None, None],    // S
    [Some(L), None, None, None, None, None],          // T
    [Some(S), Some(S), Some(L), None, None, None],    // U
    [Some(S), Some(S), Some(S), Some(L), None, None], // V
    [Some(S), Some(L), Some(L), None, None, None],    // W
    [Some(L), Some(S), Some(S), Some(L), None, None], // X
    [Some(L), Some(S), Some(L), Some(L), None, None], // Y
    [Some(L), Some(L), Some(S), Some(S), None, None], // Z
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

/// Client code can use this type to construct a different character mapping to morse code
/// and construct the decoder or encoder with this custom character set.
///
/// Empty character b' ' should be added at the beginning.
/// It does not include special characters longer than 6 signals to keep arrays small. So no $ sign for ya.
/// In order to change it and use a different mapping, client code can use [CharacterSet] type
/// to construct an array of u8 with [CHARACTER_SET_LENGTH].
/// ```ignore
/// let my_set: CharacterSet = [b' ', ...FILL IN THE CHARS...];
/// let decoder = Decoder::<128>::new().with_character_set(my_set).build();
/// ```
///
pub type CharacterSet = [u8; CHARACTER_SET_LENGTH];

/// Default international morse code characters. It includes English language letters, numbers and
/// punctuation marks.
pub const DEFAULT_CHARACTER_SET: CharacterSet = [
    b' ', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O',
    b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'1', b'2', b'3', b'4', b'5',
    b'6', b'7', b'8', b'9', b'0', b',', b'?', b':', b'-', b'"', b'(', b'=', b'X', b'.', b';', b'/',
    b'\'', b'_', b')', b'+', b'@',
];

#[cfg(feature = "decoder")]
pub mod decoder;

#[cfg(feature = "encoder")]
pub mod encoder;

pub mod message;
