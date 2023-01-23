#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]

pub enum InsertableNum
{
    Byte(u8),
    TwoByte(u16),
}

impl InsertableNum
{
    // unwrap
    // this is basically for printing purposes
    // unwraps it into a u32
    pub fn unwrap( self) -> u32
    {
        match self
        {
            InsertableNum::Byte(num) => num as u32,
            InsertableNum::TwoByte(num) => num as u32
        }
    }

    // unwrap_byte
    // returns a u8
    // will not cast this into u16s because we don't believe
    // in down casting
    pub fn unwrap_byte(self) -> u8
    {
        match self
        {
            InsertableNum::Byte(num) => num as u8,
            InsertableNum::TwoByte(num) => panic!("Down casting is frowned upon my dude")
        }
    }

    // unwrap_twobyte 
    // exptects it be two bytes
    // never panics because we believe in upcasting
    pub fn unwrap_twobyte(self) -> u16
    {
        match self
        {
            InsertableNum::TwoByte(num) => num as u16,
            InsertableNum::Byte(num) => num as u16
        }
    }


    // is_two_bytes 
    // returns true if this number is twobytes 
    pub fn is_two_bytes(self) -> bool 
    {
        match self
        {
            InsertableNum::TwoByte(_) => true,
            InsertableNum::Byte(_) => false
        }
    }
}
