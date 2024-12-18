//! Rust library for live decoding and encoding of morse code messages.
//! Supports multiple embedded devices and operating systems by being no_std.
//!
//! You can create messages by sending individual high and low signals in milliseconds to decoder,
//! from the keyboard, mouse clicks, or a button connected to some embedded device.
//! Decoder supports three precision (difficulty) modes. Lazy (easiest), Accurate (hardest) and
//! Farnsworth mode (somewhere inbetween)
//!
//! Use the encoder to turn your messages or characters into morse code strings or create a
//! sequence of signals to drive an external component such as an LED, step motor or speaker.
//!
//! # Features
//! * Decoder
//! * Encoder
//!
//! UTF-8 is supported behind a feature flag.
//! When not used it should not interfere with embedded device applications.
//!
//! The lib is no_std outside testing to make sure it will work on embedded devices
//! as well as operating systems.

// There're debug println!() statements in various parts of
// the code marked by a "// DBG" sign on top. In order to use them on a development environment
// with a proper OS and std, comment out the below attribute and uncomment the debug lines you want.

#![cfg_attr(not(test), no_std)]

#[cfg(not(feature = "utf8"))]
pub type Character = u8;

#[cfg(feature = "utf8")]
pub type Character = char;

// This is the array length for a sequence of morse signals or
// character representation of those signals while encoding
const MORSE_ARRAY_LENGTH: usize = 6;
const LONG_SIGNAL_MULTIPLIER: u16 = 3;
const WORD_SPACE_MULTIPLIER: u16 = 7;

/// We use this character to fill message arrays so when we encounter this char
/// it actually means there's no character there.
///
/// The character # is not a part of international morse code, so it's a good candidate.
pub const FILLER: Character = '#' as Character;

/// Char version of the [FILLER] coz why not? It's mainly used while generating bytes from
/// &str slices. A [char] which is utf-8 by default in Rust, can be more than one byte.
/// In ASCII mode, turning chars into bytes if they're only ascii makes sense.
pub const FILLER_CHAR: char = '#';

/// If a decoding error happens, we put this character as a placeholder.
pub const DECODING_ERROR_CHAR: Character = '?' as Character;

/// Building block of morse characters.
///
/// This enum can be used with the decoder to directly add signals to characters.
#[derive(Clone, Debug, PartialEq)]
pub enum MorseSignal {
    Short,
    Long,
}

type MorseCodeArray = [Option<MorseSignal>; MORSE_ARRAY_LENGTH];

/// This corresponds to empty character ' ' which is the default character
pub const MORSE_DEFAULT_CHAR: MorseCodeArray = [None, None, None, None, None, None];

pub mod charsets;
pub use charsets::{
    CharacterSet,
    MorseCodeSet,
    DEFAULT_CHARACTER_SET_LENGTH,
    DEFAULT_CHARACTER_SET,
    DEFAULT_MORSE_CODE_SET,
};

#[cfg(feature = "decoder")]
pub mod decoder;

#[cfg(feature = "encoder")]
pub mod encoder;

pub mod message;
