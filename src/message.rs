//! Message struct to hold decoded message or message to be encoded.
//!
//! Client code can use this to access and manipulate the
//! internal message of [MorseDecoder] or [MorseEncoder]:
//!
//! ```ignore
//! ```rust
//! // Get a decoded message
//! let decoded_message = decoder.message.as_str();
//! let decoded_message_bytes = decoder.message.as_bytes();
//! // ...Do something with the message...
//!
//! // Clear the message
//! decoder.message.clear();
//!
//! // Set message to something different
//! // and continue decoding from the end
//! decoder.set_message("SOME INITIAL MESSAGE", true);
//!
//! // We continue sending signals
//! decoder.signal_event(125, true);
//! decoder.signal_event(200, false);
//! ....
//!
//! // To show an editing cursor on the screen
//! let editing_position = decoder.message.get_edit_pos();
//! ```

use crate::{FILLER_BYTE, FILLER_CHAR};

pub struct Message<const MSG_MAX: usize> {
    chars: [u8; MSG_MAX],
    edit_pos: usize,
    clamp_edit_pos: bool,
}

impl<const MSG_MAX: usize> Default for Message<MSG_MAX> {
    fn default() -> Self {
        Self {
            chars: [FILLER_BYTE; MSG_MAX],
            edit_pos: 0,
            clamp_edit_pos: false,
        }
    }
}

// Constructor with a starter string
impl<const MSG_MAX: usize> Message<MSG_MAX> {
    /// Maximum index editing position can be at
    pub const POS_MAX: usize = MSG_MAX - 1;

    /// Get an instance of Message starting from an &str.
    ///
    /// edit_pos_end means client code wants to continue editing this
    /// text at the end.
    pub fn new(message_str: &str, edit_pos_end: bool, clamp_edit_pos: bool) -> Self {
        let mut new_self = Self {
            chars: Self::str_to_bytes(message_str),
            clamp_edit_pos,
            ..Self::default()
        };

        if edit_pos_end {
            new_self.edit_pos = new_self.len().clamp(0, Self::POS_MAX);
        }

        new_self
    }

    // Static member utility function to convert an &str to byte array internal format.
    fn str_to_bytes(str: &str) -> [u8; MSG_MAX] {
        let mut str_iter = str.chars()
            .take(MSG_MAX)
            .filter(|ch| ch.is_ascii());

        core::array::from_fn(|_|
            str_iter.next()
                .unwrap_or(FILLER_CHAR)
                .to_ascii_uppercase() as u8
        )
    }
}

// Private stuff
impl<const MSG_MAX: usize> Message<MSG_MAX> {
    // Index of last character before the last FILLER_BYTEs
    fn last_char_index(&self) -> Option<usize> {
        self.chars.iter().rposition(|ch| *ch != FILLER_BYTE)
    }

    // Check if any FILLER_BYTE characters are between normal chars
    // and convert them to ' ' space characters.
    fn update_empty_chars(&mut self) {
        if let Some(last_index) = self.last_char_index() {
            self.chars.iter_mut().enumerate().for_each(|(index, ch)| {
                if *ch == FILLER_BYTE && index < last_index {
                    *ch = b' ';
                }
            });
        }
    }
}

// Public API
impl<const MSG_MAX: usize> Message<MSG_MAX> {
    /// Get an iterator to the message bytes contained within.
    pub fn iter(&self) -> MessageIterator<MSG_MAX> {
        MessageIterator {
            message: self,
            index: 0,
        }
    }

    /// Sets current editing position to given value.
    pub fn set_edit_pos(&mut self, pos: usize) {
        self.edit_pos = pos.clamp(0, Self::POS_MAX);
    }

    /// Change the clamping behaviour of the edit position to
    /// wrapping (default) or clamping.
    ///
    /// With clamping set, when edit position is shifted to left or right,
    /// it won't cycle forward to maximum position or revert back to zero position,
    /// effectively remaining within the limits of the message no matter current position is.
    pub fn set_edit_position_clamp(&mut self, clamp: bool) {
        self.clamp_edit_pos = clamp;
    }

    pub fn is_edit_clamped(&self) -> bool {
        self.clamp_edit_pos
    }

    /// Returns current editing position.
    pub fn get_edit_pos(&self) -> usize {
        self.edit_pos
    }

    /// Move editing position to the left.
    /// By default it will wrap to the end if position is 0
    pub fn shift_edit_left(&mut self) {
        self.edit_pos = match self.edit_pos {
            0 => if self.clamp_edit_pos { 0 } else { Self::POS_MAX },
            p => p - 1,
        }
    }

    /// Move editing position to the right.
    /// By default it will wrap to the beginning if position is POS_MAX
    pub fn shift_edit_right(&mut self) {
        self.edit_pos = match self.edit_pos {
            p if p == Self::POS_MAX => if self.clamp_edit_pos { Self::POS_MAX } else { 0 },
            p => p + 1,
        }
    }

    /// Insert character at the editing position.
    ///
    /// If any characters before the character are [FILLER_BYTE]s
    /// They'll automatically be converted to empty characters ' '
    /// which means the user wants some space between words.
    pub fn add_char(&mut self, ch: u8) {
        self.chars[self.edit_pos] = ch;
        // This is only necessary if client code sets edit position
        // manually and adds a character after it, but hey.
        self.update_empty_chars();
    }

    /// Insert character at index.
    ///
    /// If any characters before the character are [FILLER_BYTE]s
    /// They'll automatically be converted to empty characters ' '
    /// which means the user wants some space between words.
    pub fn put_char_at(&mut self, index: usize, ch: u8) {
        self.chars[index] = ch;
        self.update_empty_chars();
    }

    /// Returns character at an index as a byte
    pub fn char_at(&self, index: usize) -> u8 {
        self.chars[index]
    }

    /// Returns current length of the message discarding empty FILLER_BYTE characters at the end.
    ///
    /// This is useful for creating ranged loops of actual characters decoded or can be encoded.
    pub fn len(&self) -> usize {
        let index = self.last_char_index();
        match index {
            Some(i) if i < MSG_MAX => i + 1,
            Some(i) if i == MSG_MAX => MSG_MAX,
            _ => 0,
        }
    }

    /// Returns true if the message is empty, false otherwise.
    ///
    /// This method discards FILLER_BYTE characters and only takes into
    /// account normal characters.
    pub fn is_empty(&self) -> bool {
        self.last_char_index().is_none()
    }

    /// Manually set the message from an &str.
    ///
    /// edit_pos_end flag means we'll continue from the end of this string when
    /// we continue decoding or encoding.
    pub fn set_message(&mut self, message_str: &str, edit_pos_end: bool) -> Result<(), &str> {
        if message_str.len() > MSG_MAX {
            Err("Message string can't be longer than MSG_MAX.")
        } else {
            self.chars = Self::str_to_bytes(message_str);

            if edit_pos_end {
                self.edit_pos = self.len().clamp(0, Self::POS_MAX);
            } else {
                self.edit_pos = 0;
            }

            Ok(())
        }
    }

    /// Returns the message as it is now in a byte array format.
    ///
    /// Note that this also includes 'empty' [FILLER_BYTE] characters.
    /// Client code can use return value of len() which is the actual length
    /// to loop through it or filter the fillers manually in a loop or iterator.
    pub fn as_bytes(&self) -> [u8; MSG_MAX] {
        self.chars
    }

    /// Returns the message as it is now as &str slice.
    ///
    /// Note that this *does not* include empty [FILLER_BYTE] characters.
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(self.chars[0..self.len()].as_ref()).unwrap()
    }

    /// Clear the message and start over.
    pub fn clear(&mut self) {
        self.chars = [FILLER_BYTE; MSG_MAX];
        self.edit_pos = 0;
    }
}

// Message iterator
pub struct MessageIterator<'a, const MSG_MAX: usize> {
    message: &'a Message<MSG_MAX>,
    index: usize,
}

impl<'a, const MSG_MAX: usize> Iterator for MessageIterator<'a, MSG_MAX> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.message.chars.len() {
            let result = Some(&self.message.chars[self.index]);
            self.index += 1;

            result
        } else {
            None
        }
    }
}
