/**
 * Protocol is:
 * <header><len><data><checksum><footer>
 * where:
 * - header is always 0x01
 * - len is always 2 bytes, specified as low byte, high byte
 * - data is some binary data, depending on len
 * - checksum is a 2 byte value, specified as low byte, high byte
 * - footer is always 0x00
 *
 * The crc is calculated over the length and data bytes
 */
use std::io::{self, Read};

const HEADER: u8 = 0x01;
const FOOTER: u8 = 0x00;

const BUFFERSIZE: usize = 8; //make this any size you like > 0

#[derive(Debug)]
enum States {
    GetHeader,
    LenLow,
    LenHigh,
    GetData,
    CrcLow,
    CrcHigh,
    GetFooter
}

enum ReturnTypes {
    GotMessage,
    Continue
}

struct Context {
    curr_state: States,
    data: Vec<u8>,
    expected_len: u16,
    curr_pos: u16,
    received_crc: u16,
    calculated_crc: u16,
}

impl Context {
    fn new() -> Self {
        Context {
            curr_state: States::GetHeader,
            data: Vec::new(),
            expected_len: 0,
            curr_pos: 0,
            received_crc: 0,
            calculated_crc: 0
        }
    }

    fn reset(&mut self) {
        self.curr_state = States::GetHeader;
        self.data.clear();
        self.expected_len = 0;
        self.curr_pos = 0;
        self.received_crc = 0;
        self.calculated_crc = 0;
    }
}

fn update_crc(curr_crc: u16, b: u8) -> u16 {
    /* perform some crc calculation here */
    return 0x00;
}

fn protocol(ctx: &mut Context, b: u8) -> ReturnTypes {
    //println!("got byte {:02X}, state: {:?}", b, ctx.curr_state);
    match ctx.curr_state {
        States::GetHeader => {
            if b == HEADER {
                ctx.curr_state = States::LenLow;
                ctx.calculated_crc = 0x00; //initialize crc appropriately
            }
        }
        States::LenLow => {
            ctx.expected_len = b as u16;
            ctx.calculated_crc = update_crc(ctx.calculated_crc, b);
            ctx.curr_state = States::LenHigh;
        }

        States::LenHigh => {
            ctx.expected_len |= (b as u16) << 8;
            //println!("expected len is: {}", ctx.expected_len);
            if ctx.expected_len == 0 {
                ctx.curr_state = States::GetHeader;
                return ReturnTypes::Continue;
            }
            ctx.calculated_crc = update_crc(ctx.calculated_crc, b);
            ctx.curr_pos = 0;
            ctx.data.clear();
            ctx.curr_state = States::GetData;
        }

        States::GetData => {
            ctx.data.push(b);
            ctx.curr_pos += 1;

            ctx.calculated_crc = update_crc(ctx.calculated_crc, b);

            if ctx.curr_pos == ctx.expected_len {
                ctx.curr_state = States::CrcLow;
                return ReturnTypes::Continue;
            }
        }

        States::CrcLow => {
            ctx.received_crc = b as u16;
            ctx.curr_state = States::CrcHigh;
        }

        States::CrcHigh => {
            ctx.received_crc |= (b as u16) << 8;
            if ctx.calculated_crc != ctx.received_crc {
                /* crc problem. Invalid message */
                return ReturnTypes::Continue;
            }
            ctx.curr_state = States::GetFooter;
        }

        States::GetFooter => {
            if b == FOOTER {
                return ReturnTypes::GotMessage;
            }
            ctx.curr_state = States::GetHeader;
        }
    }
    return ReturnTypes::Continue;
}

fn show_data(data: &Vec<u8>) {
    println!("Got message: ({} bytes)", data.len());

    for j in 0..data.len() {
        print!("{} ", data[j]);
        if j > 0 && (j % 16) == 0 {
            println!();
        }
    }
    println!();
}

fn main() -> io::Result<()> {
    let mut b: [u8; BUFFERSIZE] = [0x00; BUFFERSIZE];
    let mut ctx = Context::new();
    loop {
        let size = io::stdin().read(&mut b)?;
        for j in 0..size {
            let ret = protocol(&mut ctx, b[j]);
            match ret {
                ReturnTypes::GotMessage => {
                    show_data(&mut ctx.data);
                    ctx.reset();
                }
                ReturnTypes::Continue => {
                    /* don't have a full message yet */
                }
            }
        }
    }

    Ok(())
}
