#[macro_use]
extern crate vst;
extern crate rand;

use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::api::Events;
use rand::random;

#[derive(Default)]
struct Whisper {
    // Added a counter in our plugin struct. 
    notes: u8
}

// We're implementing a trait `Plugin` that does all the VST-y stuff for us.
impl Plugin for Whisper {
    fn get_info(&self) -> Info {
        Info {
            name: "Whisper".to_string(),

            // Used by hosts to differentiate between plugins.
            unique_id: 1337, 

            // We don't need inputs, but we're putting one in anyways because of a weird 
            // thing with `zip`.
            inputs: 2,

            // We do need two outputs though.  This is default, but let's be 
            // explicit anyways.
            outputs: 2,

            // Set our category
            category: Category::Synth,

            // We don't care about other stuff, and it can stay default.
            ..Default::default()
        }
    }

    // Here's the function that allows us to receive events
    fn process_events(&mut self, events: &Events) {

        // Some events aren't MIDI events - so let's do a match
        // to make sure we only get MIDI, since that's all we care about.
        for event in events.events() {
            match event {
                Event::Midi(ev) => {

                    // Check if it's a noteon or noteoff event.
                    // This is difficult to explain without knowing how the MIDI standard works.
                    // Basically, the first byte of data tells us if this signal is a note on event
                    // or a note off event.  You can read more about that here: 
                    // https://www.midi.org/specifications/item/table-1-summary-of-midi-message
                    match ev.data[0] {

                        // if note on, increment our counter
                        144 => self.notes += 1u8,

                        // if note off, decrement our counter
                        128 => self.notes -= 1u8,
                        _ => (),
                    }
                    // if we cared about the pitch of the note, it's stored in `ev.data[1]`.
                },
                // We don't care if we get any other type of event
                _ => (),
            }
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        
        // We loop through all buffers.  We specified that we will have
        // 2 inputs, and we'll have 2 outputs.  `buffer.zip()` makes sure 
        // we loop through everything.
        for (input_buffer, output_buffer) in buffer.zip() {

            // Now, we loop through each individual sample in each buffer.
            // We use an underscore `_` for the first value in our tuple, as
            // it will contain input samples (which don't exist).  The underscore
            // lets us use our conventional `zip` method while still showing we 
            // don't want to use the variable.
            for (_, output_sample) in input_buffer.iter().zip(output_buffer) {

                // if our notes incrementer is greater than 0, that means a note is currently being pressed.
                if self.notes > 0 {

                    // Finally, we change the value of each individual sample if a note is on
                    *output_sample = (random::<f32>() - 0.5f32) * 2f32;
                }
            }
        }
    }
}

plugin_main!(Whisper); 