use circular_buffer::{CircularBuffer, Error};

fn main() {
    let mut buffer: CircularBuffer<&str> = CircularBuffer::new(5);
    // let mut buffer: CircularBuffer<_> = CircularBuffer::new(5);

    buffer.write("uno");
    buffer.write("due");
    buffer.write("tre");
    buffer.write("quattro");
    buffer.write("cinque");
    buffer.write("sei"); // full
    
    buffer.overwrite("SETTE");

    println!("{:?}", buffer.read());
    println!("{:?}", buffer.read());
    println!("{:?}", buffer.read());
    println!("{:?}", buffer.read());
    println!("{:?}", buffer.read());
    println!("{:?}", buffer.read()); // empty
    println!("{:?}", buffer.read()); // empty

/*
    buffer.write(1);
    buffer.write(2);
    buffer.write(3);
    buffer.write(4);
    buffer.write(5);
    buffer.write(6); // full

    buffer.overwrite(77);

    println!("{:?}", buffer.read());
    println!("{:?}", buffer.read());
    println!("{:?}", buffer.read());
    println!("{:?}", buffer.read());
    println!("{:?}", buffer.read());
    println!("{:?}", buffer.read()); // empty
    println!("{:?}", buffer.read()); // empty
*/
}