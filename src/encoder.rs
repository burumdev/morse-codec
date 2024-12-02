//! Morse code encoder to turn text into morse code text or signals.
//!
//! The encoder takes [&str] literals or character bytes and
//! turns them into a fixed length char array. Then client code can encode these characters
//! to morse code either character by character, from slices, or all in one go.  
//! Encoded morse code can be retrieved as morse character arrays ie. ['.','-','.'] or Signal
//! Duration Multipliers [SDMArray] to calculate individual signal durations by the client code.
//!
//! This module is designed to be no_std compliant so it also should work on embedded platforms.
//!
//! ```rust
//! use morse_codec::encoder::Encoder;
//!
//! const MSG_MAX: usize = 3;
//! let mut encoder = Encoder::<MSG_MAX>::new()
//!    // We have the message to encode ready and pass it to the builder.
//!    // We pass true as second parameter to tell the encoder editing will
//!    // continue from the end of this first string.
//!    .with_message("SOS", true)
//!    .build();
//!
//! // Encode the whole message
//! encoder.encode_message_all();
//!
//! let encoded_charrays = encoder.get_encoded_message_as_morse_charrays();
//!
//! encoded_charrays.for_each(|charray| {
//!    for ch in charray.unwrap().iter()
//!        .filter(|ch| ch.is_some()) {
//!            print!("{}", ch.unwrap() as char);
//!        }
//!
//!    print!(" ");
//! });
//!
//! // This should print "... --- ..."

use crate::{
    message::Message,
    CharacterSet,
    MorseCodeArray,
    MorseSignal::{Long as L, Short as S},
    MORSE_CODE_SET,
    DEFAULT_CHARACTER_SET,
    MORSE_ARRAY_LENGTH,
    MORSE_DEFAULT_CHAR,
    LONG_SIGNAL_MULTIPLIER,
    WORD_SPACE_MULTIPLIER,
    Character,
};

const DIT: Character = '.' as Character;
const DAH: Character = '-' as Character;
const WORD_DELIMITER: Character = '/' as Character;
const SDM_LENGTH: usize = 12;

/// Signal Duration Multiplier can be 1x (short), 3x (long) or 7x (word space).
/// SDM signals are either High, or Low which corresponds to
/// electrically closed active signals or spaces inbetween them.
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum SDM {
    Empty,
    High(u8),
    Low(u8),
}

use SDM::{Empty as SDMEmpty, High as SDMHigh, Low as SDMLow};

pub type MorseCharray = [Option<Character>; MORSE_ARRAY_LENGTH];

/// Signal Duration Multipliers are arrays of u8 values
/// which can be used to multiply by a short signal duration constant
/// to calculate durations of all signals in a letter or message.
///
/// This makes it easier to write code that plays audio
/// signals with lenghts of these durations or create visual
/// representations of morse code.
pub type SDMArray = [SDM; SDM_LENGTH];

pub struct Encoder<const MSG_MAX: usize> {
    // User defined
    message: Message<MSG_MAX>,
    character_set: CharacterSet,
    // Internal stuff
    encoded_message: [&'static MorseCodeArray; MSG_MAX],
}

impl<const MSG_MAX: usize> Default for Encoder<MSG_MAX> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const MSG_MAX: usize> Encoder<MSG_MAX> {
    pub fn new() -> Self {
        Self {
            message: Message::default(),
            character_set: DEFAULT_CHARACTER_SET,
            encoded_message: [&MORSE_DEFAULT_CHAR; MSG_MAX],
        }
    }

    /// Build encoder with a starting message.
    ///
    /// edit_pos_end means we'll continue encoding from the end of this string.
    /// If you pass false to it, we'll start from the beginning.
    pub fn with_message(mut self, message_str: &str, edit_pos_end: bool) -> Self {
        self.message = Message::new(message_str, edit_pos_end, self.message.is_edit_clamped());

        self
    }

    /// Build encoder with an arbitrary editing start position.
    ///
    /// Maybe client code saved the previous editing position to an EEPROM, harddisk, local
    /// storage in web and wants to continue from that.
    pub fn with_edit_position(mut self, pos: usize) -> Self {
        self.message.set_edit_pos(pos);

        self
    }

    /// Use a different character set than default english alphabet.
    ///
    /// This can be helpful to create a message with trivial encryption.
    /// Letters can be shuffled for example. With utf-8 feature flag, a somewhat
    /// stronger encryption can be used. These kind of encryptions can
    /// easily be broken with powerful algorithms and AI.
    /// **DON'T** use it for secure communication.
    pub fn with_character_set(mut self, character_set: CharacterSet) -> Self {
        self.character_set = character_set;

        self
    }

    /// Change the wrapping behaviour of message position to clamping.
    ///
    /// This will prevent the position cycling back to 0 when overflows or
    /// jumping forward to max when falls below 0. Effectively limiting the position
    /// to move within the message length from 0 to message length maximum without jumps.
    ///
    /// If at one point you want to change it back to wrapping again:
    ///
    /// ```ignore
    /// encoder.message.set_edit_position_clamp(false);
    /// ```
    pub fn with_message_pos_clamping(mut self) -> Self {
        self.message.set_edit_position_clamp(true);

        self
    }

    /// Build and get yourself a shiny new [MorseEncoder].
    ///
    /// The ring is yours now...
    pub fn build(self) -> MorseEncoder<MSG_MAX> {
        let Encoder {
            message, character_set, encoded_message,
        } = self;

        MorseEncoder::<MSG_MAX> {
            message, character_set, encoded_message,
        }
    }
}

pub struct MorseEncoder<const MSG_MAX: usize> {
    // User defined
    pub message: Message<MSG_MAX>,
    character_set: CharacterSet,
    // Internal stuff
    encoded_message: [&'static MorseCodeArray; MSG_MAX],
}

// Private internal methods
impl<const MSG_MAX: usize> MorseEncoder<MSG_MAX> {
    fn get_morse_char_from_char(&self, ch: &Character) -> Option<&'static MorseCodeArray> {
        let index = self.character_set
            .iter()
            .position(|setchar| setchar == ch);

        if let Some(i) = index {
            Some(&MORSE_CODE_SET[i])
        } else {
            None
        }
    }

    fn get_encoded_char_as_morse_charray(&self, index: usize) -> Option<MorseCharray> {
        if index < self.message.len() {
            let encoded_char = self.encoded_message[index].clone();
            if encoded_char == MORSE_DEFAULT_CHAR {
                Some([Some(WORD_DELIMITER), None, None, None, None, None])
            } else {
                Some(encoded_char.map(|mchar| {
                    match mchar {
                        Some(S) => Some(DIT),
                        Some(L) => Some(DAH),
                        _ => None,
                    }
                }))
            }
        } else {
            None
        }
    }

    fn get_encoded_char_as_sdm(&self, index: usize) -> Option<SDMArray> {
        if index < self.message.len() {
            let mut sdm_array = [SDMEmpty; SDM_LENGTH];

            let encoded_char = self.encoded_message[index].clone();
            if encoded_char == MORSE_DEFAULT_CHAR {
                sdm_array[0] = SDMLow(WORD_SPACE_MULTIPLIER as u8);
            } else {
                let mut sdm_iter = sdm_array.iter_mut();
                let mut encoded_iter = encoded_char.iter().filter(|mchar| mchar.is_some()).peekable();

                while let Some(mchar) = encoded_iter.next() {
                    *sdm_iter.next().unwrap() = match mchar {
                        Some(S) => SDMHigh(1),
                        Some(L) => SDMHigh(LONG_SIGNAL_MULTIPLIER as u8),
                        _ => SDMEmpty,
                    };

                    // If we have a character in the future, we put a
                    // signal space between this signal and the next.
                    if encoded_iter.peek().is_some() {
                        *sdm_iter.next().unwrap() = SDMLow(1);
                    }
                }

                // Put a character ending long signal at the end.
                *sdm_iter.next().unwrap() = SDMLow(LONG_SIGNAL_MULTIPLIER as u8);
            }

            Some(sdm_array)
        } else {
            None
        }
    }

    #[cfg(not(feature = "utf8"))]
    fn encode(&mut self, ch: &Character, index: usize) -> Result<Character, &'static str> {
        if ch.is_ascii() {
            let ch_upper = ch.to_ascii_uppercase();
            match self.get_morse_char_from_char(&ch_upper) {
                Some(mchar) => {
                    self.encoded_message[index] = mchar;

                    Ok(ch_upper)
                },
                None => Err("Encoding error: Could not find character in character set.")
            }
        } else {
            Err("Encoding error: Character is not ASCII")
        }
    }

    #[cfg(feature = "utf8")]
    fn encode(&mut self, ch: &Character, index: usize) -> Result<Character, &'static str> {
        let mut ch_upper = ch.to_uppercase();

        if let Some(ch) = ch_upper.next() {
            match self.get_morse_char_from_char(&ch) {
                Some(mchar) => {
                    self.encoded_message[index] = mchar;

                    Ok(ch)
                },
                None => Err("Encoding error: Could not find character in character set.")
            }
        } else {
            Err("Encoding error: Could not convert character to uppercase.")
        }
    }
}

// Public API
impl<const MSG_MAX: usize> MorseEncoder<MSG_MAX> {
    // INPUTS

    /// Encode a single character at the edit position
    /// and add it both to the message and encoded_message.
    pub fn encode_character(&mut self, ch: &Character) -> Result<(), &str> {
        let pos = self.message.get_edit_pos();

        if pos < MSG_MAX {
            let ch_uppercase = self.encode(ch, pos);

            match ch_uppercase {
                Ok(ch) => {
                    self.message.add_char(ch);

                    // If message position is clamping then this should not do anything
                    // at the end of message position.
                    // If wrapping then it should reset the position to 0, so above condition
                    // should pass next time.
                    self.message.shift_edit_right();

                    Ok(())
                },
                Err(err) => Err(err)
            }
        } else {
            Ok(())
        }
    }

    /// Encode a &str slice at the edit position
    /// and add it both to the message and encoded message.
    ///
    /// Note if the slice exceeds maximum message length it will return an error.
    /// Non-ASCII characters will be ignored.
    #[cfg(not(feature = "utf8"))]
    pub fn encode_slice(&mut self, str_slice: &str) -> Result<(), &str> {
        let ascii_count = str_slice.chars().filter(|ch| ch.is_ascii()).count();

        if self.message.len() + ascii_count < MSG_MAX {
            str_slice.chars()
                .filter(|ch| ch.is_ascii())
                .for_each(|ch| {
                    let byte = ch as u8;
                    self.encode_character(&byte).unwrap();
                });

            Ok(())
        } else {
            Err("String slice length exceeds maximum message length.")
        }
    }

    #[cfg(feature = "utf8")]
    pub fn encode_slice(&mut self, str_slice: &str) -> Result<(), &str> {
        if self.message.len() + str_slice.len() < MSG_MAX {
            str_slice.chars()
                .for_each(|ch| {
                    self.encode_character(&ch).unwrap();
                });

            Ok(())
        } else {
            Err("String slice length exceeds maximum message length.")
        }
    }

    /// Encode the entire message from start to finish
    /// and save it to encoded_message.
    pub fn encode_message_all(&mut self) {
        for index in 0..self.message.len() {
            let ch = &self.message.char_at(index).clone();

            self.encode(ch, index).unwrap();
        }
    }

    // OUTPUTS
    /// Get last encoded message character as `Option<u8>` byte arrays of morse code.
    ///
    /// Arrays will have a fixed length of `MORSE_ARRAY_LENGTH` and if there's no
    /// signal the option will be None.
    pub fn get_last_char_as_morse_charray(&self) -> Option<MorseCharray> {
        let pos = self.message.get_edit_pos();
        if pos > 0 {
            self.get_encoded_char_as_morse_charray(pos - 1)
        } else {
            None
        }
    }

    /// Get last encoded message character as `Option<SDM>` arrays of morse code.
    ///
    /// The multiplier values then can be used to calculate durations of individual
    /// signals to play or animate the morse code.
    /// It'll be great to filter-out `Empty` values of SDM arrays.
    pub fn get_last_char_as_sdm(&self) -> Option<SDMArray> {
        let pos = self.message.get_edit_pos();
        if pos > 0 {
            self.get_encoded_char_as_sdm(pos - 1)
        } else {
            None
        }
    }

    /// Get an iterator to encoded message as `Option<u8>` byte arrays of morse code.
    /// Arrays will have a fixed length of `MORSE_ARRAY_LENGTH` and if there's no
    /// signal the option will be `None`. So it will be good to filter them out.
    pub fn get_encoded_message_as_morse_charrays(&self) -> impl Iterator<Item = Option<MorseCharray>> + '_ {
        (0..self.message.len()).map(|index| {
            self.get_encoded_char_as_morse_charray(index)
        })
    }

    /// Get an iterator to entire encoded message as `Option<SDM>` arrays of morse code.
    /// The multiplier values then can be used to calculate durations of individual
    /// signals to play or animate the morse code.
    /// It'll be good to filter `Empty` values that might fill the arrays at the end.
    pub fn get_encoded_message_as_sdm_arrays(&self) -> impl Iterator<Item = Option<SDMArray>> + '_ {
        (0..self.message.len()).map(|index| {
            self.get_encoded_char_as_sdm(index)
        })
    }
}
