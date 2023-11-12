use circular_buffer::{CircularBuffer, Error};

fn main() {
    // let mut buffer: CircularBuffer<&str> = CircularBuffer::new(5);
    let mut buffer: CircularBuffer<_> = CircularBuffer::new(5);

    buffer.write("Ciao").unwrap();
    buffer.overwrite("Mondo!");
    println!("{}", buffer.read().unwrap());
    println!("{}", buffer.read().unwrap());
}