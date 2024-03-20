use std::io::{self, Read, Write};

use bedrock::{Arena, Buffer};

const MEM_SIZE: usize = INCOMING_SIZE + OUTGOING_SIZE;
const INCOMING_SIZE: usize = 0x1000;
const OUTGOING_SIZE: usize = 0x1000;

fn main() -> io::Result<()> {
    let mem = &mut vec![0; MEM_SIZE];
    let mut arena = Arena::new(mem);

    let mut incoming = Buffer::new(arena.bytes(INCOMING_SIZE));
    let mut outgoing = Buffer::new(arena.bytes(OUTGOING_SIZE));

    let mut input = io::stdin();
    let mut output = io::stdout();

    while incoming.try_write_with(|buf| input.read(buf))? > 0 {
        incoming.read_with(|buf| outgoing.write(buf));

        outgoing.try_read_with(|buf| output.write(buf))?;
    }

    Ok(())
}
