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


fn main() {
    println!("Starting the can-over-zmq broker");
    let (_context, broadcaster, listener) = initialize_zmq();
    
    loop {
        //TODO: some flag should be added here, when we want to gracefully exit

        let msg = listener.recv_bytes(0).unwrap(); //TODO: check what this flags set, TODO: Error handling!

        println!("Got a message, sending it to all connected clients!");

        println!("Message is {:?}", msg);

        broadcaster.send(&msg, 0).unwrap(); //TODO: checks flags and error handling
    }
}