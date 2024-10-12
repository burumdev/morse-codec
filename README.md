<p align="center" style="padding: 25px 0">
  <img width="279" height="240" style="max-width: 279px" src="https://raw.githubusercontent.com/burumdev/morse-codec/refs/heads/master/morse-logo.png" alt="morse-codec logo" />
</p>

# morse-codec
Rust library for live decoding and encoding of morse code messages. Supports multiple embedded devices and operating systems by being no_std.

## Summary
You can create messages by sending individual high and low signals in milliseconds to decoder,
from the keyboard, mouse clicks, or a button connected to some embedded device.

Use the encoder to turn your messages or characters into morse code strings or create a
sequence of signals from the encoder to drive an external component such as an LED, step motor or speaker.

UTF-8 is not supported at the moment due to memory limitations of embedded devices,
but can be implemented behind a feature flag in the future. It'll be a feature in the future.

The lib is no_std outside testing to make sure it will work on embedded devices
as well as operating systems.

## Features

* **Decoder**

Live decoder for morse code that converts morse code to ASCII characters. Supports real-time decoding of incoming signals and decoding
prepared morse signals.

Receives morse signals and decodes them character by character
to create a byte array message with constant max length.
Trade-offs to support no_std include:
* No vectors or any other type of dynamic heap memory used, all data is plain old stack arrays.
* We decode the signals character by character instead of creating a large buffer for all
  signals and decoding it at the end. As a result, if an initial reference short duration is not
  provided, there is a problem with words starting with 'T' decoding as different characters.

  One way to fix the wrong decoding problems of 'T' character is to provide an initial reference short signal
  length to the decoder. A good intermediate value is 100 milliseconds.

```rust
use morse_codec::decoder::Decoder;

const MSG_MAX: usize = 64;
let mut decoder = Decoder::<MSG_MAX>::new()
    .with_reference_short_ms(90)
    .build();

// We receive high signal from button. 100 ms is a short dit signal.
decoder.signal_event(100, true);
// We receive a low signal from the button. 80 ms low signal is a signal space dit.
decoder.signal_event(80, false);
// 328 ms high long signal is a dah.
decoder.signal_event(328, true);
// 412 ms low long signal will end the character.
decoder.signal_event(412, false);
// At this point the character will be decoded and added to the message.

// Resulting character will be 'A' or '.-' in morse code.
let message = decoder.message.as_str();
assert_eq!(message, "A");
```


* **Encoder**

Morse code encoder to turn text into morse code text or signals.

Encoder takes **&str** literals or character bytes and
turns them into a fixed length byte array. Then client code can encode these characters
to morse code either one by one, from slices, or all in one go.

Encoded morse code can be retrieved as morse character arrays ie. ['.','-','.'] or Signal
Duration Multipliers **SDMArray** to calculate individual signal durations by the client code.

```rust
use morse_codec::encoder::Encoder;

const MSG_MAX: usize = 3;
let mut encoder = Encoder::<MSG_MAX>::new()
   // We have the message to encode ready and pass it to the builder.
   // We pass true as second parameter to tell the encoder editing will
   // continue from the end of this first string.
   .with_message("SOS", true)
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

// This should print "... --- ..."
```

## Running Tests
Instead of reference implementation in examples directory, we have integration tests.
They serve a dual purpose of testing the library as well as being a reference client implementation.
In addition to documentation, one can use these integration tests to understand how the library works
in practice.

Use of `--nocapture` option with the test command is recommended. This will
enable `println!()` outputs from the tests so that inputs and outputs can be observed.

```
cargo test decoding_live_lazy -- --nocapture
```

## Contributing
Contributions are more than welcome. Check the TODO section of this readme for details on currently planned features.
If you encounter bugs, you can open a 'bug' tagged issue on the Issues tab. Now you have an issue with me and the library.
If you solved the issue already then you can open a pull request with the fix.

If you have different ideas for the lib you can use the Issues tab again with an 'enhancement' tagged issue.
Please do this before sending a pull request with the new feature. This will help understand the benefits of the
new feature before incorporating it and reduces the back and forth dialogue required while reviewing the code.

All contributions will be licensed under the terms of MIT license shipped with this library.

## TODO
* <strike>Make edit position cycling to the beginning optional. Currently edit position cycles to the beginning when overflows.</strike>
* Support UTF-8 character set behind a feature flag that doesn't hurt embedded devices.
* Support playing audio of encoded messages behind a feature flag.
* Support [Farnsworth](https://www.arrl.org/files/file/Technology/x9004008.pdf) learning mode. Similar to lazy mode but more standardized.
This only involves gaps between letters and words so it can be a third option between Lazy mode and Accurate.

## License
This work is licensed under terms of MIT license as described here: https://opensource.org/license/mit

