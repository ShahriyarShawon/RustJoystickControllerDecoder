use std::env;
use std::fs;
use std::io::Read;

const JS_EVENT_BUTTON: u8 = 0x01; /* button pressed/released */
const JS_EVENT_AXIS: u8 = 0x02;  /* joystick moved */

// ensure struct is layed out how it would be in c
// more of a familiarity thing for me 
#[repr(C)]
struct js_event {
    time: u32, 
    value: i16,
    event_type: u8,
    axis_number: u8
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    
    let mut event: &js_event = &js_event { time:  0, value: 0, event_type: 0, axis_number: 0 };
    let mut file = fs::File::open(filename).expect("ok");
    let mut buffer: [u8; 8] = [0; 8];
    loop {
        file.read(&mut buffer).expect("Could not read");
        // unsafe memory wizardry
        // https://stackoverflow.com/questions/36061560/can-i-take-a-byte-array-and-deserialize-it-into-a-struct
        unsafe {
            // get raw pointer of buffer and turn it into a js_event pointer
            let event_ptr: *const js_event = buffer.as_ptr() as *const js_event;
            // not exactly sure what this means
            // I read it as a reference to an js_event pointer but it seems like
            // I dont need need to dereference to access fields
            event = &*event_ptr;
        }
        println!("{}", event.time);
        if event.event_type == JS_EVENT_BUTTON{
            println!("Button Pressed");
        }
        if event.event_type == JS_EVENT_AXIS{
            println!("Joystick Moved");
        }
        println!("Axis/Button Number: {}\nValue: {}\n\n",event.axis_number, event.value);
    }
}
