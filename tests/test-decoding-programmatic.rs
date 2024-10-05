use morse_codec::{
    decoder::{
        Decoder,
        Precision,
    },
    CharacterSet,
    MorseSignal::{ Long as L, Short as S }, FILLER_BYTE,
};

#[test]
fn direct_signal_entry_sos() {
    const MESSAGE_MAX_LENGTH: usize = 3;
    let mut decoder = Decoder::<MESSAGE_MAX_LENGTH>::new()
        .with_precision(Precision::Accurate).build();

    // S character is Short Short Short
    decoder.add_signal_to_character(Some(S));
    decoder.add_signal_to_character(Some(S));
    decoder.add_signal_to_character(Some(S));

    decoder.add_current_char_to_message();

    // O character is Long Long Long
    decoder.add_signal_to_character(Some(L));
    decoder.add_signal_to_character(Some(L));
    decoder.add_signal_to_character(Some(L));

    decoder.add_current_char_to_message();

    // S character is Short Short Short
    decoder.add_signal_to_character(Some(S));
    decoder.add_signal_to_character(Some(S));
    decoder.add_signal_to_character(Some(S));

    decoder.add_current_char_to_message();

    let message_length = decoder.message.len();
    println!("Message length: {:?}", message_length);

    let message = decoder.message.as_bytes();
    for i in 0..message_length {
        println!("Message letter: {}", message[i] as char);
    }

    assert_eq!(message, [b'S', b'O', b'S']);
}

// Create a message containing two SOS words separated by a word space
// ie: "SOS SOS".
#[test]
fn decoding_double_sos() {
    const MESSAGE_MAX_LENGTH: usize = 8;

    let mut decoder = Decoder::<MESSAGE_MAX_LENGTH>::new()
        .with_precision(Precision::Accurate).build();

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

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
    decoder.signal_event(100, true);
    decoder.signal_event(700, false);

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

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
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    let message_length = decoder.message.len();
    println!("Message length: {:?}", message_length);

    let message = decoder.message.as_bytes();
    for i in 0..message_length {
        println!("Message letter: {}", message[i] as char);
    }

    assert_eq!(message, [b'S', b'O', b'S', b' ', b'S', b'O', b'S', FILLER_BYTE]);
}

// Create a message with the words "INM TES ETS SET ET TE E T"
// E and T letters are particularly problematic while decoding dynamically
// because they're single dot or dash letters with a follow-up space.
#[test]
fn decoding_bug_prone() {
    const MESSAGE_MAX_LENGTH: usize = 32;

    let mut decoder = Decoder::<MESSAGE_MAX_LENGTH>::new()
        .with_precision(Precision::Accurate).build();

    // ----------------------------
    // I
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    // N
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    // M
    decoder.signal_event(300, true);
    decoder.signal_event(100, false);
    decoder.signal_event(300, true);
    decoder.signal_event(700, false);

    // ----------------------------
    // T
    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    // E
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    // S
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(700, false);

    // ----------------------------
    // E
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    // T
    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    // S
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(700, false);

    // ----------------------------
    // S
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    // E
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    // T
    decoder.signal_event(300, true);
    decoder.signal_event(700, false);

    // ----------------------------
    // E
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    // T
    decoder.signal_event(300, true);
    decoder.signal_event(700, false);

    // ----------------------------
    // T
    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    // E
    decoder.signal_event(100, true);
    decoder.signal_event(700, false);

    // ----------------------------
    // E
    decoder.signal_event(100, true);
    decoder.signal_event(700, false);

    // ----------------------------
    // T
    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    let message_length = decoder.message.len();
    println!("Message length: {:?}", message_length);

    let message = decoder.message.as_bytes();
    for i in 0..message_length {
        println!("Message letter: {}", message[i] as char);
    }

    assert_eq!(
        message,
        [
            b'I', b'N', b'M', b' ', b'T', b'E', b'S', b' ', b'E',
            b'T', b'S', b' ', b'S', b'E', b'T', b' ', b'E', b'T',
            b' ', b'T', b'E', b' ', b'E', b' ', b'T',
            FILLER_BYTE, FILLER_BYTE, FILLER_BYTE, FILLER_BYTE, FILLER_BYTE, FILLER_BYTE, FILLER_BYTE
        ]
    );
}

// Create a message with a single "E"
// This one should work if everything works as planned.
#[test]
fn decoding_single_e() {
    const MESSAGE_MAX_LENGTH: usize = 1;

    let mut decoder = Decoder::<MESSAGE_MAX_LENGTH>::new()
        .with_precision(Precision::Accurate).build();

    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    let message_length = decoder.message.len();
    println!("Message length: {:?}", message_length);

    let message = decoder.message.as_bytes();
    for i in 0..message_length {
        println!("Message letter: {}", message[i] as char);
    }

    assert_eq!(message[0] as char, 'E');
}

// Create a message with a single "T"
// This one is a showstopper
// How can we be sure if a long signal followed by a long signal space is T or E,
// if we didn't sample any signals before it?
// FIXME: Challenge accepted? Fix it.
#[test]
fn decoding_single_t() {
    const MESSAGE_MAX_LENGTH: usize = 1;

    let mut decoder = Decoder::<MESSAGE_MAX_LENGTH>::new()
        .with_precision(Precision::Accurate).build();

    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    let message_length = decoder.message.len();
    println!("Message length: {:?}", message_length);

    let message = decoder.message.as_bytes();
    for i in 0..message_length {
        println!("Message letter: {}", message[i] as char);
    }

    assert_eq!(message[0] as char, 'T');
}

// Create a message with a single "T"
// This time we use a reference short signal duration
// passed to the builder.
// So this should work as expected.
#[test]
fn decoding_single_t_default_short_ms() {
    const MESSAGE_MAX_LENGTH: usize = 1;

    let mut decoder = Decoder::<MESSAGE_MAX_LENGTH>::new()
        .with_precision(Precision::Accurate).with_reference_short_ms(100).build();

    decoder.signal_event(300, true);
    decoder.signal_event(300, false);

    let message_length = decoder.message.len();
    println!("Message length: {:?}", message_length);

    let message = decoder.message.as_bytes();
    for i in 0..message_length {
        println!("Message letter: {}", message[i] as char);
    }

    assert_eq!(message[0] as char, 'T');
}

#[test]
fn decoding_sos_with_custom_character_set() {
    const MESSAGE_MAX_LENGTH: usize = 3;

    let character_set: CharacterSet = [
        b' ',
        b'I', b'U', b'C', b'E', b'D', b'F', b'Z', b'P', b'A', b'J', b'K', b'X', b'T', b'N', b'V', b'H', b'Q', b'S', b'R',
        b'M', b'B', b'O', b'W', b'L', b'Y', b'G',
        b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0',
        b',', b'?', b':', b'-', b'"', b'(', b'=', b'X', b'.', b';', b'/', b'\'', b'_', b')', b'+', b'@',
    ];

    println!("TEST DECODING WITH CUSTOM CHARACTER SET:");

    println!();
    character_set.iter().for_each(|ch| print!("{} ", *ch as char));
    println!();

    let mut decoder = Decoder::<MESSAGE_MAX_LENGTH>::new()
        .with_character_set(character_set).build();

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

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
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    let message_length = decoder.message.len();
    println!("Message length: {:?}", message_length);

    let message = decoder.message.as_bytes();
    for i in 0..message_length {
        println!("Message letter: {}", message[i] as char);
    }

    assert_eq!(message, [b'R', b'V', b'R']);
}

#[test]
fn decoding_with_starter_message() {
    const MESSAGE_MAX_LENGTH: usize = 128;

    println!("TEST DECODING STARTING FROM MESSAGE STR");
    println!("We add SOS to the end of a message.");

    let mut decoder = Decoder::<MESSAGE_MAX_LENGTH>::new()
        .with_message("Some message starter: ", true).build();

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

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
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    let message_length = decoder.message.len();
    println!("Message length: {:?}", message_length);

    let message = decoder.message.as_bytes();
    for i in 0..message_length {
        println!("Message letter: {}", message[i] as char);
    }

    assert_eq!(message.into_iter().take(message_length).rev().collect::<Vec<u8>>()[..3], [b'S', b'O', b'S']);

    println!("We set the message again to some text, but start from the beginning this time.");

    decoder.message.set_message("Hey there starting over now.", false).unwrap();

    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(100, false);
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

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
    decoder.signal_event(100, true);
    decoder.signal_event(300, false);

    let message_length = decoder.message.len();
    println!("Message length: {:?}", message_length);

    let message = decoder.message.as_bytes();
    for i in 0..message_length {
        println!("Message letter: {}", message[i] as char);
    }

    assert_eq!(message[..9], [b'S', b'O', b'S', b' ', b'T', b'H', b'E', b'R', b'E']);
}

#[test]
fn set_get_message_str() {
    const MESSAGE_MAX_LENGTH: usize = 128;

    println!("TEST PUSHING PULLING MESSAGE AS STR");

    let mut decoder = Decoder::<MESSAGE_MAX_LENGTH>::new()
        .with_message("Start", true).build();

    println!("Got message back: {}", decoder.message.as_str());
    println!("Message length: {}", decoder.message.len());
    println!("Edit position is at: {}", decoder.message.get_edit_pos());

    println!("Rewriting message with another message");
    decoder.message.set_message("Some long message...", true).unwrap();

    println!();

    println!("Got message back after rewrite: {}", decoder.message.as_str());
    println!("Message length after rewrite: {}", decoder.message.len());
    println!("Edit position after rewrite is at: {}", decoder.message.get_edit_pos());

    println!();

    println!("Rewriting message with an illegal utf-8 message");

    decoder.message.set_message("Some message with utf-8: french Élysée (like Elysee) pallace and spanish señor (like senor)", true).unwrap();

    println!("Got message back after rewrite: {}", decoder.message.as_str());
    println!("Message length after rewrite: {}", decoder.message.len());
    println!("Edit position after rewrite is at: {}", decoder.message.get_edit_pos());
}
