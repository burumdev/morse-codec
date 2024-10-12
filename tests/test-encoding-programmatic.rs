use std::{
    thread,
    time::Duration,
};

use morse_codec::encoder::{
    Encoder,
    MorseCharray,
    SDM,
};

const QUICK_FOX: &str = "The quick brown fox jumps over the lazy dog?";

fn print_morse_charray(mchar: MorseCharray) {
    for ch in mchar.iter().filter(|ch| ch.is_some()) {
        print!("{}", ch.unwrap() as char);
    }
    print!(" ");
}

#[test]
fn encoding_sos_one_by_one() {
    const MESSAGE_MAX_LENGTH: usize = 32;

    println!("TESTING ENCODING 'SOS SOS'");

    let mut encoder = Encoder::<MESSAGE_MAX_LENGTH>::new().build();

    encoder.encode_character(&b'S').unwrap();
    print_morse_charray(encoder.get_last_char_as_morse_charray().unwrap());
    encoder.encode_character(&b'O').unwrap();
    print_morse_charray(encoder.get_last_char_as_morse_charray().unwrap());
    encoder.encode_character(&b'S').unwrap();
    print_morse_charray(encoder.get_last_char_as_morse_charray().unwrap());
    encoder.encode_character(&b' ').unwrap();
    print_morse_charray(encoder.get_last_char_as_morse_charray().unwrap());
    encoder.encode_character(&b'S').unwrap();
    print_morse_charray(encoder.get_last_char_as_morse_charray().unwrap());
    encoder.encode_character(&b'O').unwrap();
    print_morse_charray(encoder.get_last_char_as_morse_charray().unwrap());
    encoder.encode_character(&b'S').unwrap();
    print_morse_charray(encoder.get_last_char_as_morse_charray().unwrap());

    println!();
}

#[test]
fn encoding_fox_one_by_one() {
    const MESSAGE_MAX_LENGTH: usize = 64;

    println!("TESTING ENCODING 'The quick brown fox...' by iteration");
    println!("Message string is: {}", QUICK_FOX);
    println!();
    println!("Morse encoded version:");

    let mut encoder = Encoder::<MESSAGE_MAX_LENGTH>::new().build();

    QUICK_FOX.bytes().for_each(|ch| {
        encoder.encode_character(&ch).unwrap();
        print_morse_charray(encoder.get_last_char_as_morse_charray().unwrap());
    });

    println!();
}

#[test]
fn encoding_fox_whole() {
    const MESSAGE_MAX_LENGTH: usize = 64;

    println!("TESTING ENCODING 'The quick brown fox...' with whole string operation");
    println!();

    let mut encoder = Encoder::<MESSAGE_MAX_LENGTH>::new()
        .with_message(QUICK_FOX, true).build();

    println!("Message string is: {}", encoder.message.as_str());
    println!("Message length: {}", encoder.message.len());
    println!("Morse encoded version:");
    println!();

    encoder.encode_message_all();
    let encoded_charrays = encoder.get_encoded_message_as_morse_charrays();

    encoded_charrays.for_each(|charray| {
        print_morse_charray(charray.unwrap());
    });

    println!();
    println!("After encoding message string is: {}", encoder.message.as_str());
    println!("Message length: {}", encoder.message.len());

    println!();

    println!("Adding a long slice at the end to trigger error. But we don't quit because it's a test.");
    let result = encoder.encode_slice(" Adding something at the end.");

    match result {
        Ok(_) => {
            let encoded_charrays = encoder.get_encoded_message_as_morse_charrays();
            encoded_charrays.for_each(|charray| {
                print_morse_charray(charray.unwrap());
            });
        },
        Err(err) => {
            println!("Error: {}", err);
        }
    }

    let result = encoder.encode_slice(". And lands safely.");

    match result {
        Ok(_) => {
            println!("Added some more message at the end.");
            println!();
            let encoded_charrays = encoder.get_encoded_message_as_morse_charrays();
            encoded_charrays.for_each(|charray| {
                print_morse_charray(charray.unwrap());
            });
        },
        Err(err) => {
            println!("Error: {}", err);
        }
    }

    println!();
    println!("Now message is longer with length: {} and message: {}", encoder.message.len(), encoder.message.as_str());
    encoder.message.iter().for_each(|ch| {
        print!("{} ", *ch as char);
    });
    println!();
}

#[test]
fn encoding_fox_sdm() {
    const MESSAGE_MAX_LENGTH: usize = 64;

    println!("TEST ENCODING 'The quick brown fox...' as message and get signal duration multipliers");
    println!();

    let mut encoder = Encoder::<MESSAGE_MAX_LENGTH>::new()
        .with_message(QUICK_FOX, true).build();

    println!("Message string is: {}", encoder.message.as_str());
    println!("Message length: {}", encoder.message.len());
    println!("Morse encoded version:");
    println!();

    encoder.encode_message_all();
    let encoded_charrays = encoder.get_encoded_message_as_morse_charrays();

    encoded_charrays.for_each(|charray| {
        print_morse_charray(charray.unwrap());
    });

    println!();
    println!();

    println!("Morse encoding signal duration multipliers:");

    use std::time::Instant;
    let start_time = Instant::now();

    let encoded_sdms = encoder.get_encoded_message_as_sdm_arrays();

    println!("Elapsed time encoding: {:.2?}", start_time.elapsed());

    encoded_sdms.for_each(|sdm| {
        println!("{:?}", sdm);
    });

    encoder.encode_character(&b'?').unwrap();

    let last_charray = encoder.get_last_char_as_morse_charray().unwrap();
    let last_sdm = encoder.get_last_char_as_sdm().unwrap();

    println!("? mark added and morse charray is:\n{:?}", last_charray);
    println!("? mark added and sdm is:\n{:?}", last_sdm);
}

#[test]
fn encoding_fox_play_sdm() {
    const MESSAGE_MAX_LENGTH: usize = 64;
    const SHORT_DURATION: u16 = 50;

    println!("TEST ENCODING 'The quick brown fox...' as message and 'play' the signal duration multipliers we get.");
    println!();

    let mut encoder = Encoder::<MESSAGE_MAX_LENGTH>::new()
        .with_message(QUICK_FOX, false).build();

    println!("Message string is: {}", encoder.message.as_str());
    println!("Message length: {}", encoder.message.len());
    println!("Morse encoded version:");
    println!();

    encoder.encode_message_all();
    let encoded_charrays = encoder.get_encoded_message_as_morse_charrays();

    encoded_charrays.for_each(|charray| {
        print_morse_charray(charray.unwrap());
    });

    println!();
    println!("PLAYING");

    let encoded_sdms = encoder.get_encoded_message_as_sdm_arrays();

    encoded_sdms.for_each(|sdm_array| {
        sdm_array.unwrap().iter()
            .filter(|&&sdm| sdm != SDM::Empty)
            .for_each(|sdm| {
                let duration = match sdm {
                    SDM::High(mul) => {
                        let d = *mul as u16 * SHORT_DURATION;
                        println!("HIGH! ({})", d);

                        d
                    },
                    SDM::Low(mul) => {
                        let d = *mul as u16 * SHORT_DURATION;
                        println!("LOW! ({})", d);

                        d
                    },
                    _ => 0,
                };

                thread::sleep(Duration::from_millis(duration as u64));
            });
    });
}

#[test]
fn message_position_clamping() {
    const MSG_MAX: usize = 4;

    println!("TESTING ENCODING WITH A CLAMPED EDIT POSITION");
    println!("Message max length is {}", MSG_MAX);
    println!();

    let mut encoder = Encoder::<MSG_MAX>::new().with_message_pos_clamping().build();

    encoder.encode_character(&b'R').unwrap();
    encoder.encode_character(&b'U').unwrap();
    encoder.encode_character(&b'S').unwrap();
    encoder.encode_character(&b'T').unwrap();

    let message = encoder.message.as_str();

    println!("Message in the encoder: {}", message);

    assert_eq!(message, "RUST");

    let encoded_charrays = encoder.get_encoded_message_as_morse_charrays();
    println!("Message as morse code:");
    encoded_charrays.for_each(|charray| print_morse_charray(charray.unwrap()));

    println!();

    encoder.encode_character(&b'.').unwrap();
    let message = encoder.message.as_str();
    println!("Message in the encoder after adding a dot: {}", message);

    assert_eq!(message, "RUS.");

    println!();
    println!("We clear the message and restart with a wrapping behaviour this time.");
    encoder.message.clear();
    encoder.message.set_edit_position_clamp(false);

    encoder.encode_character(&b'R').unwrap();
    encoder.encode_character(&b'U').unwrap();
    encoder.encode_character(&b'S').unwrap();
    encoder.encode_character(&b'T').unwrap();

    let message = encoder.message.as_str();

    println!("Message in the wrapping encoder: {}", message);

    assert_eq!(message, "RUST");

    let encoded_charrays = encoder.get_encoded_message_as_morse_charrays();
    println!("Message in wrapping encoder as morse code:");
    encoded_charrays.for_each(|charray| print_morse_charray(charray.unwrap()));

    println!();

    encoder.encode_character(&b'.').unwrap();
    let message = encoder.message.as_str();
    println!("Message in the wrapping encoder after adding a dot: {}", message);

    assert_eq!(message, ".UST");

    let encoded_charrays = encoder.get_encoded_message_as_morse_charrays();
    println!("Message in wrapping encoder as morse code:");
    encoded_charrays.for_each(|charray| print_morse_charray(charray.unwrap()));
}
