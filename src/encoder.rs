//! Morse code encoder to turn text into morse code text or signals.
//!
//! The encoder takes [&str] literals or character bytes and
//! turns them into a fixed length char array. Then client code can encode these characters
//! to morse code either character by character, from slices, or all in one go.  
//! Encoded morse code can be retrieved as morse character arrays ie. ['.','-','.'] or Signal
//! Duration Multipliers [SDMArray] to calculate individual signal durations by the client code.
//!
//! This module is designed to be no_std compliant so it also should work on embedded platforms.

use crate::{
    message::Message,
    CharacterSet,
    MorseCodeArray,
    MorseSignal::{ Long as L, Short as S },
    DEFAULT_CHARACTERS,
    LONG_SIGNAL_MULTIPLIER,
    MORSE_ARRAY_LENGTH,
    MORSE_CHARACTERS,
    MORSE_DEFAULT_CHAR,
    WORD_SPACE_MULTIPLIER,
};

const DIT: u8 = b'.';
const DAH: u8 = b'-';
const WORD_DELIMITER: u8 = b'/';
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

use SDM::{ Empty as SDMEmpty, High as SDMHigh, Low as SDMLow };

pub type MorseCharray = [Option<u8>; MORSE_ARRAY_LENGTH];

/// Signal Duration Multipliers are arrays of u8 values
/// which can be used to multiply by a short signal duration constant
/// to calculate durations of all signals in a letter or message.
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
            character_set: DEFAULT_CHARACTERS,
            encoded_message: [&MORSE_DEFAULT_CHAR; MSG_MAX],
        }
    }

    pub fn with_message(mut self, message_str: &str, edit_pos_end: bool) -> Self {
        self.message = Message::new(message_str, edit_pos_end);

        self
    }

    pub fn with_edit_position(mut self, pos: usize) -> Self {
        self.message.set_edit_pos(pos);

        self
    }

    pub fn with_character_set(mut self, character_set: CharacterSet) -> Self {
        self.character_set = character_set;

        self
    }

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
    fn get_morse_char_from_char(&self, ch: &u8) -> Option<&'static MorseCodeArray> {
        let index = self.character_set
            .iter()
            .position(|setchar| setchar == ch);

        if let Some(i) = index {
            Some(&MORSE_CHARACTERS[i])
        } else {
            //TODO: Maybe convert this into a Result with a custom error struct, or am I
            // asking for trouble?
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

                    // If we have a character in the future, we put a space between
                    // this signal and the next.
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

    fn encode(&mut self, ch: &u8, index: usize) {
        let ch_upper = ch.to_ascii_uppercase();
        match self.get_morse_char_from_char(&ch_upper) {
            Some(mchar) => {
                self.message.add_char(ch_upper);
                self.encoded_message[index] = mchar;
            },
            //TODO: Handle character not found case here. We currently do nothing.
            None => ()
        }
    }
}

// Public API
impl<const MSG_MAX: usize> MorseEncoder<MSG_MAX> {

    // INPUTS
    pub fn encode_character(&mut self, ch: &u8) -> Result<(), &str> {
        let pos = self.message.get_edit_pos();
        if pos < MSG_MAX {
            self.encode(ch, pos);
            self.message.shift_edit_right();

            Ok(())
        } else {
            Err("Maximum message length reached.")
        }
    }

    pub fn encode_slice(&mut self, str_slice: &str) -> Result<(), &str> {
        if self.message.get_edit_pos() + str_slice.bytes().len() < MSG_MAX {
            str_slice.bytes().for_each(|ch| {
                self.encode_character(&ch).unwrap();
            });

            Ok(())
        } else {
            Err("String slice length exceeds maximum message length.")
        }
    }

    pub fn encode_message_all(&mut self) {
        for index in 0..self.message.len() {
            let ch = &self.message.char_at(index).clone();
            self.encode(ch, index);
        }

        //TODO: It will be better to return a unit result here ie. Result<(), &str>
    }

    // OUTPUTS
    pub fn get_last_char_as_morse_charray(&self) -> Option<MorseCharray> {
        let pos = self.message.get_edit_pos();
        if pos > 0 {
            self.get_encoded_char_as_morse_charray(pos - 1)
        } else {
            None
        }
    }

    pub fn get_last_char_as_sdm(&self) -> Option<SDMArray> {
        let pos = self.message.get_edit_pos();
        if pos > 0 {
            self.get_encoded_char_as_sdm(pos - 1)
        } else {
            None
        }
    }

    pub fn get_encoded_message_as_morse_charrays(&self) -> impl Iterator<Item = Option<MorseCharray>> + '_ {
        (0..self.message.len()).map(|index| {
            self.get_encoded_char_as_morse_charray(index)
        })
    }

    pub fn get_encoded_message_as_sdm_arrays(&self) -> impl Iterator<Item = Option<SDMArray>> + '_ {
        (0..self.message.len()).map(|index| {
            self.get_encoded_char_as_sdm(index)
        })
    }
}
