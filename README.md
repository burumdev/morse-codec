# morse-codec
Rust library for live decoding and encoding of morse code messages. Supports multiple embedded devices and operating systems by being no_std.

## Summary
You can create messages by sending individual high and low signals in milliseconds to decoder,
from the keyboard, mouse clicks, or a button connected to some embedded device.
You can also bypass signal input and add prepared short or long morse signals to characters
directly.

Use the encoder to turn your messages or characters into morse code strings or create a
sequence of signals from the encoder to drive an external component such as an LED, step motor or speaker.

UTF-8 is not supported at the moment due to memory limitations of embedded devices,
but can be implemented behind a feature flag in the future.

The lib is no_std outside testing to make sure it will work on embedded devices
as well as operating systems.

## Features

* Decoder

Live decoder for morse code that converts morse code to ASCII characters. Supports real-time decoding of incoming signals and decoding
prepared morse signals.

Receives morse signals and decodes them character by character
to create a char array (charray) message with constant max length.
Empty characters will be filled with the const FILLER_BYTE and
decoding errors will be filled with DECODING_ERROR_BYTE.
Trade-offs to support no_std include:
* No vectors or any other type of dynamic heap memory used, all data is plain old stack arrays.
* We decode the signals character by character instead of creating a large buffer for all
  signals and decoding it at the end. As a result, if an initial reference short duration is not
  provided, there is a problem with words starting with 'T' decoding as different characters.

  One way to fix the wrong decoding problems of 'T' character is to provide an initial reference short signal
  length to the decoder. A good intermediate value is 100 milliseconds.

```rust
const MSG_MAX = 64;
let decoder = morse_codec::Decoder::<MSG_MAX>::new()
    .with_reference_short_ms(90)
    .build();

// We receive high signal from button. 100 ms is a short dit signal
// because reference_short_ms is 90 and default tolerance range factor is 0.5.
// 90 ms falls into 100 x 0.5 = 50 ms to 100 + 50 = 150 ms.
// So it's a short or dit signal.
decoder.signal_event(100, true);
// We receive a low signal from the button. 80 ms low signal is a signal space dit.
// It falls between 50 and 150.
decoder.signal_event(80, false);
// 328 ms high long signal is a dah. 328 x 0.5 = 164, 328 + 164 = 492.
// Reference short signal 90 x 3 (long signal multiplier) = 270. 270 falls into the range.
decoder.signal_event(328, true);
// 412 ms low long signal will end the character.
decoder.signal_event(412, false);
// At this point the character will be decoded and added to the message.

// Resulting character will be 'A' or '.-' in morse code.

```

* Encoder

Morse code encoder to turn text into morse code text or signals.

The encoder takes **&str** literals or character bytes and
turns them into a fixed length char array. Then client code can encode these characters
to morse code either character by character, from slices, or all in one go.  
Encoded morse code can be retrieved as morse character arrays ie. ['.','-','.'] or Signal
Duration Multipliers **SDMArray** to calculate individual signal durations by the client code.

```rust
const MSG_MAX = 3;
let encoder = morse_codec::Encoder::<MSG_MAX>::new()
    // We have the message to encode ready and pass it to the builder
    .with_message("SOS")
    .build();

// Encode the whole message
encoder.encode_message_all();

let encoded_charrays = encoder.get_encoded_message_as_morse_charrays();

encoded_charrays.for_each(|charray| {
    for ch in charray.unwrap().iter()
        .filter(|ch| ch.is_some()) {
            print!("{}", ch.unwrap() as char);
        }

    print!(" ");
});

```

## Running Tests
Instead of reference implementation in an examples directory, we have integration tests.
They serve a dual purpose of testing the library as well as being a reference client implementation.
In addition to the documentation, one can use these integration tests to learn how the library works
in practice.

While running tests use of `--nocapture` option with the test command is recommended. This will
enable `println!()` outputs from the tests so inputs and outputs can be observed.

```
cargo test test_decoding_live_lazy -- --nocapture
```

## License
This work is licensed under terms of MIT license as described here: https://opensource.org/license/mit

