//! Contains morse code to character set mappings.

use crate::{
    MorseSignal::{Long as L, Short as S},
    MORSE_DEFAULT_CHAR,
    MorseCodeArray,
    Character,
};

/// Maximum number of characters in default mapping set of morse code to letters.
pub const DEFAULT_CHARACTER_SET_LENGTH: usize = 53;

/// Allows creating a custom character set.
///
/// Client code can use this type to construct a different character mapping to morse code
/// and construct the decoder or encoder with this custom character set.
///
/// Empty character b' ' should be added at the beginning.
/// It does not include special characters longer than 6 signals to keep arrays small. So no $ sign for ya.
/// In order to change it and use a different mapping, client code can use [CharacterSet] type
/// to construct an array of u8 with [CHARACTER_SET_LENGTH].
/// ```ignore
/// let my_set: CharacterSet = b" ADD SOME CHARACTERS TO THIS BYTE STRING"];
/// // Or with 'utf8' feature
/// let my_set: CharacterSet = &[' ', ...FILL IN THE CHARS...];
/// // Then
/// let decoder = Decoder::<128>::new().with_character_set(my_set).build();
/// ```
pub type CharacterSet = &'static [Character];

/// Default international morse code characters. It includes English language letters, numbers and
/// punctuation marks. In utf8 mode a custom version of this array can be used while building an Encoder or Decoder
/// using 'with_character_set' functions. Corresponding [MORSE_CODE_SET]
/// can also be changed to support different languages.
#[cfg(not(feature = "utf8"))]
pub const DEFAULT_CHARACTER_SET: CharacterSet = b" ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890,?:-\"(=X.;/'_)+@";

#[cfg(feature = "utf8")]
pub const DEFAULT_CHARACTER_SET: CharacterSet = &[
        ' ',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
        'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
        ',', '?', ':', '-', '"', '(', '=', 'X', '.', ';', '/', '\'', '_', ')', '+', '@',
    ];

/// Allows creating a custom morse code set.
///
/// Client code can use this type to construct a different morse code mapping to characters
/// and construct the decoder or encoder with this custom morse code set.
pub type MorseCodeSet = &'static [MorseCodeArray];

/// Default internal representation of morse characters.
///
/// Letters can be converted to these morse code arrays and vice-versa. To support an utf8
/// character set, this array of morse codes can be changed along with the corresponding [CharacterSet]
pub const DEFAULT_MORSE_CODE_SET: MorseCodeSet =
    &[
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

