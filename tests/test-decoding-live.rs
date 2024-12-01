
use morse_codec::decoder::{
    Decoder, Precision
};
use std::{
    thread::sleep,
    time::{ Duration, Instant },
};
use keyboard_query::{ DeviceQuery, DeviceState };

// Create a message from key press and release events dynamically
// In this case we use the 's' key for a signal event
// and take the empty spaces inbetween signals as down signals.
// 'a' key can be used to end a character at the end of a message and show the message.
// A more elegant solution to this whould be to calculate the elapsed idle time (eventless time)
// and act upon it, but for purposes of this integration test we just use 'a' for it.
// 'q' key quits the test. 
// Note that this test uses external crate 'keyboard_query' for keyboard press and release events.
// It requires X11 dev libs on linux, otherwise it might not compile. What it requires on Windows and MacOS is beyond me,
// but in theory it should work on those platforms as well.
fn decoding_live(precision: Precision, initial_short: u16) {
    println!("TESTING DECODING LIVE");
    println!("With precision: {:?}", precision);

    const MESSAGE_MAX_LENGTH: usize = 16;
    println!("Message maximum length is {}", MESSAGE_MAX_LENGTH);

    println!("\nPress 's' for a high signal, release 's' for a low signal, 'a' to end input and show the resulting message, 'q' to quit.");

    let mut decoder = Decoder::<MESSAGE_MAX_LENGTH>::new()
        .with_precision(precision)
        .with_reference_short_ms(initial_short)
        .build();

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
                        decoder.signal_event(diff as u16, false);
                    }

                    last_signal_time = Some(Instant::now());
                } else if keys[0] == 30 { // Matching character 'a' for all or end input
                    decoder.signal_event_end(false);

                    let message_length = decoder.message.len();
                    let message = decoder.message.as_charray();

                    if message_length > 0 {
                        println!();
                        print!("Message: ");
                        for &ch in message.iter().take(message_length) {
                            print!("{}", ch as char);
                        }
                        println!();
                        println!("Current speed in Words Per Minute is {}", decoder.get_wpm());
                    }
                } else if keys[0] == 16 { // Character 'q' for quitting
                    break;
                }
            } else if prev_keys.len() == 1 && prev_keys[0] == 31 && keys.is_empty() {
                let diff = last_signal_time.unwrap().elapsed().as_millis();
                //println!("SIGNAL time diff = {} ms", diff);
                decoder.signal_event(diff as u16, true);
                
                last_space_time = Some(Instant::now());
            }
        }

        prev_keys = keys;

        // Sleep for 5 milliseconds (5000000 nanoseconds) to avoid clogging up resources
        sleep(Duration::new(0, 5_000_000));
    }
}

#[test]
fn decoding_live_accurate() {
    decoding_live(Precision::Accurate, 0);
}

#[test]
fn decoding_live_lazy() {
    decoding_live(Precision::Lazy, 0);
}

#[test]
fn decoding_live_100_ms() {
    decoding_live(Precision::Lazy, 100);
}

#[test]
fn decoding_live_farnsworth_half() {
    decoding_live(Precision::Farnsworth(0.5), 100);
}

#[test]
fn decoding_live_farnsworth_quarter() {
    decoding_live(Precision::Farnsworth(0.25), 100);
}
