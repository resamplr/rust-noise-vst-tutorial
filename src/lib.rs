#[macro_use]
extern crate vst;
extern crate rand;

use vst::plugin::{Info, Plugin, Category, CanDo};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::api::{Events, Supported};
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

            // We don't need inputs
            inputs: 0,

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

        // We only want to process *anything* if a note is
        // being held.  Else, we can return early and skip
        // processing anything!
        if self.notes == 0 { return }
        
        // `buffer.split()` gives us a tuple containing the 
        // input and output buffers.  We only care about the
        // output, so we can ignore the input by using `_`.
        let (_, output_buffer) = buffer.split();

        // Now, we want to loop over our output channels.  This
        // includes our left and right channels (or more, if you
        // are working with surround sound).
        for output_channel in output_buffer.into_iter() {
            // Let's iterate over every sample in our channel.
            for output_sample in output_channel {
                // For every sample, we want to add a random value from
                // -1.0 to 1.0.
                *output_sample = (random::<f32>() - 0.5f32) * 2f32;
            }
        }
    }

    // It's good to tell our host what our plugin can do
    // Some VST hosts might not send any midi events to our plugin
    // if we don't explicitely tell them that the plugin can handle them.
    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            // Tell our host that the plugin supports receiving MIDI messages
            CanDo::ReceiveMidiEvent => Supported::Yes,
            // Maybe it also supports ather things
            _ => Supported::Maybe,
        }
    }
}

plugin_main!(Whisper); 

