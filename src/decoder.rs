//! Live decoder for morse code that converts morse code to ASCII characters. Supports real-time decoding of incoming signals and decoding
//! prepared morse signals.
//!
//! Receives morse signals and decodes them character by character
//! to create a char array (charray) message with constant max length.
//! Empty characters will be filled with the const FILLER_BYTE and
//! decoding errors will be filled with DECODING_ERROR_BYTE.
//! Trade-offs to support no_std include:
//! * No vectors or any other type of dynamic heap memory used, all data is plain old stack arrays.
//! * We decode the signals character by character instead of creating a large buffer for all
//!   signals and decoding it at the end. As a result, if an initial reference short duration is not
//!   provided, we have problems with words starting with 'T' decoding as different characters. This is a problem because
//!   we can't determine the length of spaces (low signals) after the high signal being long with only one signal as reference.
//!   Creating a large buffer would fix this, because we could audit the entire signal buffer to iron out wrong decodings,
//!   but the large size of the buffer would not fit into small RAM capacities of certain 8 bit
//!   MCUs like AVR ATmega328P with SRAM size of 2KB and even smaller sizes for simpler chips. So we
//!   clear the buffer every time we add a character.
//!   One way to fix the wrong decoding problems of 'T' character is to provide an initial reference short signal
//!   length to the decoder. A good intermediate value is 100 milliseconds.
//!
//! ```
//! const MSG_MAX = 64;
//! let decoder = morse_codec::Decoder::<MSG_MAX>::new()
//!     .with_reference_short_ms(90)
//!     .build();
//!
//! // We receive high signal from button. 100 ms is a short dit signal because reference_short_ms is 90
//! // ms, default tolerance range factor is 0.5. 90 ms falls into 100 x 0.5 = 50 ms to 100 + 50 = 150 ms.
//! // So it's a short or dit signal.
//! decoder.signal_event(100, true);
//! // We receive a low signal from the button. 80 ms low signal is a signal space dit.
//! // It falls between 50 and 150.
//! decoder.signal_event(80, false);
//! // 328 ms high long signal is a dah. 328 x 0.5 = 164, 328 + 164 = 492.
//! // Reference short signal 90 x 3 (long signal multiplier) = 270. 270 falls into the range.
//! decoder.signal_event(328, true);
//! // 412 ms low long signal will end the character.
//! decoder.signal_event(412, false);
//! // At this point the character will be decoded and added to the message.
//!
//! // Resulting character will be 'A' or '.-' in morse code.
//!
//! ```
//!

use core::ops::RangeInclusive;

use crate::{
    message::Message,
    CharacterSet, MorseCodeArray,
    MorseSignal::{self, Long as L, Short as S},
    DECODING_ERROR_BYTE, DEFAULT_CHARACTERS, LONG_SIGNAL_MULTIPLIER, MORSE_ARRAY_LENGTH,
    MORSE_CHARACTERS, MORSE_DEFAULT_CHAR, WORD_SPACE_MULTIPLIER,
};

/// Decoding precision is either Lazy or Accurate.
///
/// If Lazy is selected, short and long signals will be considered to saturate their
/// fields on the extreme ends. For example a short signal can be 1 ms to short range end
/// and a long signal is from this point to the start of a very long (word separator) signal.
/// If Accurate is selected, short and long signals will only be decoded correctly if they fall into a range
/// of lower tolerance value and higher tolerance value. Default value for tolerance factor is 0.5.
/// So if a short signal is expected to be 100 ms, correct decoding signal can be anywhere between
/// 50 ms to 150 ms, but not 10 ms.
/// Default precision is Lazy, as it's the most human friendly precision.
#[derive(Debug, PartialEq)]
pub enum Precision {
    Lazy,
    Accurate,
}
use Precision::{Accurate, Lazy};

type MilliSeconds = u16;

#[derive(PartialEq, Copy, Clone, Debug)]
enum SignalDuration {
    Empty,
    Short(MilliSeconds),
    Long(MilliSeconds),
    Other(MilliSeconds),
}
use SignalDuration::{Empty as SDEmpty, Long as SDLong, Other as SDOther, Short as SDShort};

// Signal buffer length is morse array length + 1, because we need to be able to
// resolve a character ending long signal (either 3x or word space 7x) at the end
// of each character.
const SIGNAL_BUFFER_LENGTH: usize = MORSE_ARRAY_LENGTH + 1;
type SignalBuffer = [SignalDuration; SIGNAL_BUFFER_LENGTH];

const SINGLE_SHORT_SIGNAL_MORSE_CHAR: MorseCodeArray = [Some(S), None, None, None, None, None];
const SINGLE_LONG_SIGNAL_MORSE_CHAR: MorseCodeArray = [Some(L), None, None, None, None, None];

struct LastSingleSignalLetter {
    index: usize,
    duration: MilliSeconds,
}

/// This is the builder, or public interface of the decoder using builder pattern.
/// It builds a MorseDecoder which is the concrete implementation and returns it with build().
/// For details on how to use the decoder, refer to [MorseDecoder] documentation.
pub struct Decoder<const MSG_MAX: usize> {
    // User defined
    precision: Precision,
    character_set: CharacterSet,
    signal_tolerance: f32,
    reference_short_ms: MilliSeconds,
    // Internal stuff
    message: Message<MSG_MAX>,
    current_character: MorseCodeArray,
    signal_pos: usize,
    signal_buffer: SignalBuffer,
    last_single_signal_letter: Option<LastSingleSignalLetter>,
}

impl<const MSG_MAX: usize> Default for Decoder<MSG_MAX> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const MSG_MAX: usize> Decoder<MSG_MAX> {
    pub fn new() -> Self {
        Self {
            // User defined
            precision: Lazy,
            character_set: DEFAULT_CHARACTERS,
            signal_tolerance: 0.50,
            reference_short_ms: 0,
            // Internal stuff
            message: Message::default(),
            current_character: MORSE_DEFAULT_CHAR,
            signal_pos: 0,
            signal_buffer: [SDEmpty; SIGNAL_BUFFER_LENGTH],
            last_single_signal_letter: None,
        }
    }

    /// Build decoder with a starting message.
    ///
    /// edit_pos_end means we'll continue decoding from the end of this string.
    /// If you pass false to it, we'll start from the beginning.
    pub fn with_message(mut self, message_str: &str, edit_pos_end: bool) -> Self {
        self.message = Message::new(message_str, edit_pos_end);

        self
    }

    /// Build decoder with an arbitrary editing start position.
    ///
    /// Maybe client code saved the previous editing position to an EEPROM, harddisk, local
    /// storage in web and wants to continue from that.
    pub fn with_edit_position(mut self, pos: usize) -> Self {
        self.message.set_edit_pos(pos);

        self
    }

    /// Set decoder precision.
    ///
    /// Precision::Lazy is more human friendly, Precision::Accurate is for learning
    /// or a challenge contest.
    pub fn with_precision(mut self, precision: Precision) -> Self {
        self.precision = precision;

        self
    }

    /// Use a different character set than default english alphabet.
    ///
    /// This can be helpful to create a message with trivial encryption.
    /// Letters can be shuffled for example. This kind of encryption can
    /// easily be broken with powerful algorithms and AI.
    /// **DON'T** use it for secure communication.
    pub fn with_character_set(mut self, character_set: CharacterSet) -> Self {
        self.character_set = character_set;

        self
    }

    /// Use a different signal tolerance range factor than the default 0.5.
    ///
    /// Tolerance factors higher than 0.5 tend to overlap and result in wrong decoding.
    /// You can lower this value though for stricter morse signalling.
    /// In any case the value will be clamped between 0.0 and 1.0 so values
    /// higher than 1.0 will be 1.0.
    pub fn with_signal_tolerance(mut self, signal_tolerance: f32) -> Self {
        self.signal_tolerance = signal_tolerance.clamp(0.0, 1.0);

        self
    }

    /// Change initial reference short signal duration from 0 to some other value.
    ///
    /// This value will determine the reference durations of signal types (short, long or very long).
    /// The value will be multiplied by LONG_SIGNAL_MULTIPLIER (x3) and WORD_SPACE_MULTIPLIER (x7) to
    /// determine long signals and very long word separator signals.
    /// Default value of 0 means MorseDecoder will try to calculate the reference short duration
    /// from incoming signals. This might not work well if the message starts with a 'T'.
    pub fn with_reference_short_ms(mut self, reference_short_ms: MilliSeconds) -> Self {
        self.reference_short_ms = reference_short_ms;

        self
    }

    /// Build and get yourself a shiny new [MorseDecoder].
    ///
    /// The ring is yours now...
    pub fn build(self) -> MorseDecoder<MSG_MAX> {
        let Decoder {
            precision,
            character_set,
            signal_tolerance,
            reference_short_ms,
            message,
            current_character,
            signal_pos,
            signal_buffer,
            last_single_signal_letter,
        } = self;

        MorseDecoder::<MSG_MAX> {
            precision,
            character_set,
            signal_tolerance,
            reference_short_ms,
            message,
            current_character,
            signal_pos,
            signal_buffer,
            last_single_signal_letter,
        }
    }
}

/// This is the concrete implementation of the decoder.
///
/// It doesn't have a new function, or public data members,
/// so to get an instance of it, use public builder interface [Decoder].
pub struct MorseDecoder<const MSG_MAX: usize> {
    // User defined
    precision: Precision,
    character_set: CharacterSet,
    signal_tolerance: f32,
    reference_short_ms: MilliSeconds,
    // Internal stuff
    pub message: Message<MSG_MAX>,
    current_character: MorseCodeArray,
    signal_pos: usize,
    signal_buffer: SignalBuffer,
    last_single_signal_letter: Option<LastSingleSignalLetter>,
}

// Private stuff.. Don' look at it
impl<const MSG_MAX: usize> MorseDecoder<MSG_MAX> {
    fn get_char_from_morse_char(&self, morse_char: &MorseCodeArray) -> u8 {
        let index = MORSE_CHARACTERS
            .iter()
            .position(|mchar| mchar == morse_char);

        if let Some(i) = index {
            self.character_set[i]
        } else {
            // For now we return the DECODING_ERROR_BYTE character for unknown
            // TODO: Maybe handle this better? Return an Option or Result maybe? Anyone?
            DECODING_ERROR_BYTE
        }
    }

    fn add_to_signal_buffer(&mut self, signal_duration: SignalDuration) {
        if self.signal_pos < SIGNAL_BUFFER_LENGTH {
            self.signal_buffer[self.signal_pos] = signal_duration;
            self.signal_pos += 1;
        }
    }

    fn decode_signal_buffer(&mut self) -> MorseCodeArray {
        let mut morse_array: MorseCodeArray = MORSE_DEFAULT_CHAR;
        let mut short_ms = 0;

        //DBG
        //println!("Signal buffer decoding: {:?}", self.signal_buffer);

        self.signal_buffer
            .iter()
            .take(6)
            .enumerate()
            .for_each(|(i, signal)| match signal {
                SDShort(sms) => {
                    morse_array[i] = Some(S);
                    short_ms = *sms;
                }
                SDLong(_) => morse_array[i] = Some(L),
                _ => {}
            });

        // If the char got decoded as a single short signal, we save it to a buffer
        // for later checking if it was a single long signal instead. For example in English
        // character set; If the letter is an 'E' we save it for future checking. Maybe it's not an E but a T
        if morse_array == SINGLE_SHORT_SIGNAL_MORSE_CHAR {
            self.last_single_signal_letter = Some(LastSingleSignalLetter {
                index: self.message.get_edit_pos(),
                duration: short_ms,
            });
        }

        morse_array
    }

    fn resolve_signal_duration(
        &mut self,
        duration_ms: MilliSeconds,
        tolerance_range: &RangeInclusive<MilliSeconds>,
    ) -> SignalDuration {
        let short_tolerance_range = self.signal_tolerance_range(self.reference_short_ms);

        match self.precision {
            Lazy => {
                let short_range_end = short_tolerance_range.end() + 50; // 50 ms padding gives better results with humans
                if (0u16..short_range_end).contains(&duration_ms) {
                    SDShort(duration_ms)
                } else if (short_range_end..self.word_space_ms()).contains(&duration_ms) {
                    SDLong(duration_ms)
                } else {
                    SDOther(duration_ms)
                }
            }
            Accurate => {
                if tolerance_range.contains(&self.reference_short_ms) {
                    SDShort(duration_ms)
                } else if tolerance_range.contains(&self.long_signal_ms()) {
                    SDLong(duration_ms)
                } else {
                    SDOther(duration_ms)
                }
            }
        }
    }

    fn signal_tolerance_range(&self, duration_ms: MilliSeconds) -> RangeInclusive<MilliSeconds> {
        let diff = (duration_ms as f32 * self.signal_tolerance) as MilliSeconds;

        duration_ms - diff..=duration_ms.saturating_add(diff)
    }

    fn reset_character(&mut self) {
        self.signal_buffer = [SDEmpty; SIGNAL_BUFFER_LENGTH];
        self.signal_pos = 0;
        self.current_character = MORSE_DEFAULT_CHAR;
    }

    fn update_reference_short_ms(&mut self, duration_ms: MilliSeconds) {
        self.reference_short_ms = duration_ms;
    }

    fn long_signal_ms(&self) -> MilliSeconds {
        self.reference_short_ms * LONG_SIGNAL_MULTIPLIER
    }

    fn word_space_ms(&self) -> MilliSeconds {
        let multiplier = if self.precision == Lazy {
            9
        } else {
            WORD_SPACE_MULTIPLIER
        };

        self.reference_short_ms * multiplier
    }
}

// Public API for the masses
impl<const MSG_MAX: usize> MorseDecoder<MSG_MAX> {
    /// Returns currently resolved reference short signal duration.
    ///
    /// Reference short signal is resolved continuously by the decoder as signal events pour in.
    /// As longer signal durations are calculated by multiplying this value,
    /// it might be useful for the client code.
    pub fn get_reference_short(&self) -> MilliSeconds {
        self.reference_short_ms
    }

    /// Directly add a prepared signal to the character.
    ///
    /// Signal duration resolving is done by the client code, or you're using a prepared signal.
    pub fn add_signal_to_character(&mut self, signal: Option<MorseSignal>) {
        if self.signal_pos < MORSE_ARRAY_LENGTH {
            self.current_character[self.signal_pos] = signal;
            self.signal_pos += 1;
        }
    }

    /// Add current decoded character to the message.
    ///
    /// This happens automatically when using `signal_event` calls.
    /// Use this with `add_signal_to_character` directly with
    /// prepared [MorseSignal] enums.
    pub fn add_current_char_to_message(&mut self) {
        if self.message.get_edit_pos() < MSG_MAX {
            let ch = self.get_char_from_morse_char(&self.current_character);
            self.message.add_char(ch);
            self.message.shift_edit_right();

            self.reset_character();
        }
    }

    /// Manually end a sequence of signals.
    ///
    /// This decodes the current character and moves to the next one
    /// With end_word flag it will optionally add a space after it.
    /// Especially useful when client code can't determine if signal
    /// input by the operator ended, because no other high signal is
    /// following the low signal at the end. At that point a separate button
    /// or whatever can be used to trigger this function.
    pub fn signal_event_end(&mut self, end_word: bool) {
        self.current_character = self.decode_signal_buffer();
        self.add_current_char_to_message();

        if end_word {
            self.current_character = MORSE_DEFAULT_CHAR;
            self.add_current_char_to_message();
        }
    }

    /// Send signal events to the decoder, filling signal buffer one event at a time.
    ///
    /// When a character ending long space signal or a word ending long space is sent,
    /// signal buffer will be decoded automatically and character will be added to message.
    /// Note that if signal input itself has ended, oftentimes there's no way to send that signal.
    /// Use `signal_event_end` at that point to manually end the character.
    pub fn signal_event(&mut self, duration_ms: MilliSeconds, is_high: bool) {
        let tolerance_range = self.signal_tolerance_range(duration_ms);

        match self.signal_pos {
            // Signal is the first in the series.
            // Since this is the first signal we encounter, we'll treat it as if it's a short, when
            // reference_short_ms == 0, otherwise try to resolve signal duration based on
            // reference short learned from previous letters or based on
            // initial_reference_short_ms provided to the constructor.
            // If we have set it short preemptively, we later on check if this first short turns out to be long instead
            // (see one of the match arms). We'll update the first buffer item with the correct value then don't worry.
            0 => {
                if is_high {
                    //DBG
                    //println!("START CHARACTER -----------------------");

                    if self.reference_short_ms == 0 {
                        self.add_to_signal_buffer(SDShort(duration_ms));
                        self.update_reference_short_ms(duration_ms);

                        //DBG
                        //println!("Initial ref short is set to {}", duration_ms);
                    } else {
                        let resolved_duration = self.resolve_signal_duration(duration_ms, &tolerance_range);

                        //DBG
                        //println!("\tINTIAL HIGH: tolerance range: {:?}, position is: {}, resolved duration: {:?}, ref short is: {}", tolerance_range, pos, resolved_duration, self.reference_short_ms);

                        self.add_to_signal_buffer(resolved_duration);
                    }
                } else {
                    // Do nothing if we receive a low signal at the start of a series.
                    // This happens when event engine of the client code sends low signals
                    // inadvertently perhaps while idling or outright sends a wrong low signal at the start of a letter
                    // which is rude.
                }
            }

            // Signal is not high. It can be one of three things at this point:
            // 1. It's a short duration space signal (space between two signals)
            // 2. It's a long duration space. At this point we decode the entire signal buffer and
            // add resulting character to the message
            // 3. It's a very long signal (x7 or more) to divide two words in the message. So
            // we check the signal buffer and add the character, as well as a space after it.
            _pos if !is_high => {
                if duration_ms < self.reference_short_ms && !tolerance_range.contains(&self.reference_short_ms) {
                    //println!("Updating reference short to {}", duration_ms);
                    self.update_reference_short_ms(duration_ms);
                }

                let resolved_duration = self.resolve_signal_duration(duration_ms, &tolerance_range);

                //DBG
                //println!("LOW SIGNAL: tolerance range: {:?}, position is: {}, resolved duration: {:?}, ref short is: {}", tolerance_range, _pos, resolved_duration, self.reference_short_ms);

                match resolved_duration {
                    SDLong(_) => {
                        //DBG
                        //println!("END CHARACTER --------------");

                        if let Some(last_single) = &self.last_single_signal_letter {
                            if tolerance_range.contains(&last_single.duration) {
                                self.message.put_char_at(last_single.index, self.get_char_from_morse_char(&SINGLE_LONG_SIGNAL_MORSE_CHAR));
                                self.last_single_signal_letter = None;
                            }
                        }

                        self.signal_event_end(false);
                    }
                    SDOther(ms) if ms >= self.word_space_ms() => {
                        //DBG
                        //println!("END WORD --------------");

                        self.signal_event_end(true);
                    }
                    _ => (),
                }
            }

            // Signal is not the first in a series and there are signals to be fed to the buffer.
            // At this point signal is high and we try to resolve signal duration and save the signal to character.
            // Also we check if the first duration in the array was wrongly saved as short but
            // should be long instead. We fix it to long duration if it's wrong.
            // The reason why we check at this position starting from index 2+ is that
            // we get a better calibrated short signal from the low signal before it (index 1)
            pos if pos < SIGNAL_BUFFER_LENGTH && is_high => {
                let resolved_duration = self.resolve_signal_duration(duration_ms, &tolerance_range);

                //DBG
                //println!("\tHIGH SIGNAL: tolerance range: {:?}, position is: {}, resolved duration: {:?}, ref short is: {}", tolerance_range, pos, resolved_duration, self.reference_short_ms);

                self.add_to_signal_buffer(resolved_duration);

                if let SDShort(first_duration) = self.signal_buffer[0] {
                    match resolved_duration {
                        SDLong(_) => {
                            // If current signal is long and it's tolerance range contains the
                            // first short signal, the first short signal should be a long
                            if tolerance_range.contains(&first_duration) {
                                self.signal_buffer[0] = SDLong(duration_ms);
                            }
                        }
                        SDShort(_) => {
                            // This is an edge case we need to handle where the character being
                            // decoded, has a long high signal as the first signal in it and
                            // has only short signals after it (including this one). If tolerance range
                            // of the short signal we just got happens to be in the range of first
                            // short signal divided by long signal multiplier (by default 3),
                            // first short signal was indeed a long one, but we missed it.
                            if tolerance_range.contains(&(first_duration / LONG_SIGNAL_MULTIPLIER)) {
                                self.signal_buffer[0] = SDLong(duration_ms);
                            }
                        }
                        _ => (),
                    }
                }
            }

            // This means we got the maximum amount of signals to the buffer, but still couldn't
            // decode the character. Either because we never received a character ender low
            // signal (3x short space) or a word ending long signal (7x short space)
            // or outright couldn't decode them, but hey.
            // For now we just reset the character and start over, but we should perhaps
            //TODO: Find a way to better handle this situation. Putting a decoding error character
            // might be a better option for example
            _ => {
                //DBG
                //println!("We reached the end of buffer! Oh my god! We're gonna fall off this cliff and get an overflow!! Naah just relax..");
                self.reset_character();
            }
        }
    }
}
