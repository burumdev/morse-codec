use morse_codec::decoder::{
    Decoder,
    Precision,
};

// Create a message containing two SOS words separated by a word space
// ie: "SOS SOS".
// Then use the message iterator to iterate over them
#[test]
fn message_iter() {
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

    let message_iter = decoder.message.iter();

    for ch in message_iter {
        println!("Message letter: {}", *ch as char);
    }
}

#[test]
fn message_pop() {
    println!("TEST decoder with 'SOS', then popping the last character.");
    println!();

    use morse_codec::MorseSignal::{Long as L, Short as S};

    const MESSAGE_MAX_LENGTH: usize = 3;

    let mut decoder = Decoder::<MESSAGE_MAX_LENGTH>::new().build();

    decoder.add_signal_to_character(Some(S));
    decoder.add_signal_to_character(Some(S));
    decoder.add_signal_to_character(Some(S));

    decoder.add_current_char_to_message();

    decoder.add_signal_to_character(Some(L));
    decoder.add_signal_to_character(Some(L));
    decoder.add_signal_to_character(Some(L));

    decoder.add_current_char_to_message();

    decoder.add_signal_to_character(Some(S));
    decoder.add_signal_to_character(Some(S));
    decoder.add_signal_to_character(Some(S));

    decoder.add_current_char_to_message();

    let msg = decoder.message.as_str();

    println!("Message before popping the last char: {}", msg);
    if let Some(char) = decoder.message.pop() {
        println!("Last character is: {}", char as char);
        println!("Message after popping: {}", decoder.message.as_str());
    }

    println!();
    println!("Popping till empty");
    while let Some(char) = decoder.message.pop() {
        println!("Last character is: {}", char as char);
    }

    assert!(decoder.message.is_empty());
}

