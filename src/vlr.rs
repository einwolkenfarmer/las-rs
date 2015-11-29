//! Variable length records.

use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use super::Result;
use io::read_full;

const DEFAULT_HEADER_LENGTH: u16 = 54;

/// A variable length record
#[derive(Clone, Debug, PartialEq)]
pub struct Vlr {
    /// Reserved for future use.
    pub reserved: u16,
    /// ASCII data that identifies the record.
    pub user_id: [u8; 16],
    /// Integer id for this record type.
    pub record_id: u16,
    /// The number of bytes in the actual record data.
    pub record_length_after_header: u16,
    /// A textual description of this record.
    pub description: [u8; 32],
    /// The record data themselves.
    pub record: Vec<u8>,
}

impl Vlr {
    /// Reads a Vlr from a `Read`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use std::io::{Seek, SeekFrom};
    /// use las::header::Header;
    /// use las::vlr::Vlr;
    /// let ref mut reader = File::open("data/1.0_0.las").unwrap();
    /// let header = Header::read_from(reader).unwrap();
    /// reader.seek(SeekFrom::Start(header.header_size as u64)).unwrap();
    /// let vlr = Vlr::read_from(reader);
    /// ```
    pub fn read_from<R: Read>(reader: &mut R) -> Result<Vlr> {
        let mut vlr = Vlr::new();
        vlr.reserved = try!(reader.read_u16::<LittleEndian>());
        try!(read_full(reader, &mut vlr.user_id));
        vlr.record_id = try!(reader.read_u16::<LittleEndian>());
        vlr.record_length_after_header = try!(reader.read_u16::<LittleEndian>());
        try!(read_full(reader, &mut vlr.description));
        vlr.record = vec![0; vlr.record_length_after_header as usize];
        try!(read_full(reader, &mut vlr.record));
        Ok(vlr)
    }

    /// Creates a new, empty `Vlr`.
    ///
    /// # Examples
    ///
    /// ```
    /// use las::vlr::Vlr;
    /// let vlr = Vlr::new();
    /// ```
    pub fn new() -> Vlr {
        Vlr {
            reserved: 0,
            user_id: [0; 16],
            record_id: 0,
            record_length_after_header: 0,
            description: [0; 32],
            record: Vec::new(),
        }
    }

    /// Returns the length of this variable length record in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use las::vlr::Vlr;
    /// let vlr = Vlr::new();
    /// assert_eq!(54, vlr.len());
    /// ```
    pub fn len(&self) -> u32 {
        DEFAULT_HEADER_LENGTH as u32 + self.record.len() as u32
    }
}
