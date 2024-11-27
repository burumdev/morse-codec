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

