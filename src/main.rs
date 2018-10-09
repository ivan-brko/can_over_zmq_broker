extern crate zmq;

//TODO: move this function to it's own module
fn initialize_zmq() -> (zmq::Context, zmq::Socket, zmq::Socket) {
    let context = zmq::Context::new();
    let broadcaster = context.socket(zmq::PUB).unwrap(); //TODO: add error handling here!
    assert!(broadcaster.bind("tcp://127.0.0.1:5558").is_ok()); //TODO: handle this in a nicer way

    let listener = context.socket(zmq::PULL).unwrap(); //TODO: again, handle this in a nicer way
    assert!(listener.bind("tcp://127.0.0.1:4568").is_ok()); //TODO: same thing

    (context, broadcaster, listener)
}

fn get_id_from_message(msg: &[u8]) -> u32 {
    let mut sum: u32 = 0;
    for (i, v) in msg[..4].iter().rev().enumerate() {
        sum += 256u32.pow(i as u32) * (*v as u32);
    }
    sum
}

fn print_message(msg: &[u8]) {
    let mut printed_message = String::from("Message:\tid:");
    let id = get_id_from_message(msg);
    printed_message.push_str(&id.to_string());
    printed_message.push_str("; data: ");

    for msg_byte in msg.iter().skip(4) {
        printed_message.push_str(&format!("{:X} ", msg_byte))
    }

    println!("{}", printed_message);
}

fn main() {
    println!("Starting the can-over-zmq broker");
    let (_context, broadcaster, listener) = initialize_zmq();

    loop {
        //TODO: some flag should be added here, when we want to gracefully exit

        let msg = listener.recv_bytes(0).unwrap(); //TODO: check what this flags set, TODO: Error handling!

        print_message(&msg);

        broadcaster.send(&msg, 0).unwrap(); //TODO: checks flags and error handling
    }
}
