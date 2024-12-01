use morse_codec::{
    decoder::Decoder,
    encoder::{
        Encoder,
        MorseCharray,
        MorseEncoder,
        SDM
    }
};

#[cfg(feature = "utf8")]
use morse_codec::message::Utf8Charray;

use keyboard_query::{
    DeviceQuery,
    DeviceState,
};

use std::{
    time::{ Instant, Duration },
    thread::sleep,
};

const MSG_MAX: usize = 16;

fn print_morse_charray(mchar: MorseCharray) {
    for ch in mchar.iter().filter(|ch| ch.is_some()) {
        print!("{}", ch.unwrap() as char);
    }
    print!(" ");
}

fn play_message(morse_encoder: &mut MorseEncoder<MSG_MAX>) {
    println!("Now 'playing' reencoded message. You'll like it.");
    let sdms = morse_encoder.get_encoded_message_as_sdm_arrays();

    const SHORT_DURATION: u16 = 150;
    sdms.enumerate().for_each(|(index, sdm_array)| {
        println!("SDM array: {:?}", sdm_array);
        println!("CHARACTER IS: {}", morse_encoder.message.as_charray()[index] as char);
        sdm_array.unwrap().iter()
            .filter(|&&sdm| sdm != SDM::Empty)
            .for_each(|sdm| {
                let duration = match sdm {
                    SDM::High(mul) => {
                        let d = *mul as u16 * SHORT_DURATION;
                        println!("HIGH! ({}) ", d);

                        d
                    },
                    SDM::Low(mul) => {
                        let d = *mul as u16 * SHORT_DURATION;
                        println!("LOW! ({}) ", d);

                        d
                    },
                    _ => 0,
                };

                sleep(Duration::from_millis(duration as u64));
            });
    });
}

#[cfg(not(feature = "utf8"))]
fn reencode_message(message: &str, morse_encoder: &mut MorseEncoder<MSG_MAX>) {
    println!("*****************************");
    println!("ENCODER REENCODES THE MESSAGE");
    println!("*****************************");

    morse_encoder.message.set_message(message, false).unwrap();
    morse_encoder.encode_message_all();
    let encoded_charrays = morse_encoder.get_encoded_message_as_morse_charrays();

    println!("Reencoded message as morse string: ");
    encoded_charrays.for_each(|charray| print_morse_charray(charray.unwrap()));
    println!();
}

#[cfg(feature = "utf8")]
fn reencode_message(message: Utf8Charray, morse_encoder: &mut MorseEncoder<MSG_MAX>) {
    println!("*****************************");
    println!("ENCODER REENCODES THE MESSAGE");
    println!("*****************************");

    morse_encoder.message.clear();

    for ch in message.iter() {
        morse_encoder.encode_character(ch).unwrap();
    }
    let encoded_charrays = morse_encoder.get_encoded_message_as_morse_charrays();

    println!("Reencoded message as morse string: ");
    encoded_charrays.for_each(|charray| print_morse_charray(charray.unwrap()));
    println!();
}

#[test]
fn decode_encode_sdm() {
    println!("TESTING DECODING AND THEN ENCODING DECODED MESSAGE");
    println!("Message maximum length is: {}", MSG_MAX);

    println!("\nPress 's' for a high signal, release 's' for a low signal, 'a' to end input and show the resulting message, 'q' to quit.");

    let mut morse_decoder = Decoder::<MSG_MAX>::new()
        .with_reference_short_ms(100)
        .build();

    let mut morse_encoder = Encoder::<MSG_MAX>::new().build();

    let device_state = DeviceState::new();
    let mut prev_keys = vec![];

    let mut last_signal_time: Option<Instant> = None;
    let mut last_space_time: Option<Instant> = None;

    loop {
        let keys = device_state.get_keys();
        if keys != prev_keys {
            if keys.len() == 1 {
                if keys[0] == 31 { // Matching character 's' for signal
                    if last_space_time.is_some() {
                        let diff = last_space_time.unwrap().elapsed().as_millis();
                        //println!("SPACE time diff = {} ms", diff);
                        morse_decoder.signal_event(diff as u16, false);
                    }

                    last_signal_time = Some(Instant::now());
                } else if keys[0] == 30 { // Matching character 'a' for all or end input
                    morse_decoder.signal_event_end(false);

                    let message = morse_decoder.message.as_str();

                    println!();
                    println!("Decoder message: {}", message);
                    println!();

                    reencode_message(message, &mut morse_encoder);
                    play_message(&mut morse_encoder);
                } else if keys[0] == 16 { // Character 'q' for quitting
                    break;
                }
            } else if prev_keys.len() == 1 && prev_keys[0] == 31 && keys.is_empty() {
                let diff = last_signal_time.unwrap().elapsed().as_millis();
                //println!("SIGNAL time diff = {} ms", diff);
                morse_decoder.signal_event(diff as u16, true);

                last_space_time = Some(Instant::now());
            }
        }

        prev_keys = keys;

        // Sleep for 5 milliseconds (5000000 nanoseconds) to avoid clogging up resources
        sleep(Duration::new(0, 5_000_000));
    }
}
