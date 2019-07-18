#[macro_use]
extern crate serde_derive;

pub use bin::VectorTable as VectorTable;
pub use bin::VectorTable as BinVectorTable;
pub use json::VectorTable as JsonVectorTable;

//FIXME: Find a way to factorise the implementation of Table for the bin & json versions of the VectorTable.

mod common {
    pub use std::io;

    pub use serde::de::DeserializeOwned;
    pub use serde::Serialize;

    pub use ironsea_store::{Load, Store};
    pub use ironsea_table::Table;
}

mod bin {
    use super::common::*;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct VectorTable<R> {
        data: Vec<R>,
    }

    impl<R> VectorTable<R> {
        pub fn new(data: Vec<R>) -> Self {
            VectorTable { data }
        }

        pub fn get_mut(&mut self) -> &mut Vec<R> {
            &mut self.data
        }
    }

    impl<R> Table<R> for VectorTable<R> {
        fn get_table(&self) -> Vec<&R> {
            let mut list = vec![];
            for v in &self.data {
                list.push(v);
            }

            list
        }

        fn get_record(&self, pos: usize) -> Option<&R> {
            self.data.get(pos)
        }
    }

    impl<R> Store for VectorTable<R>
        where
            R: Serialize,
    {
        fn store<W>(&mut self, writer: W) -> io::Result<()>
            where
                W: std::io::Write,
        {
            match bincode::serialize_into(writer, &self) {
                Ok(_) => Ok(()),
                Err(e) => Err(io::Error::new(io::ErrorKind::WriteZero, e)),
            }
        }
    }

    impl<R> Load for VectorTable<R>
        where
            R: DeserializeOwned,
    {
        fn load<Re: io::Read>(reader: Re) -> io::Result<Self> {
            match bincode::deserialize_from(reader) {
                Ok(data) => Ok(data),
                Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
            }
        }

        // only required for store_mapped_file
        fn load_slice(from: &[u8]) -> io::Result<Self> {
            match bincode::deserialize(from) {
                Ok(data) => Ok(data),
                Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
            }
        }
    }
}

mod json {
    use super::common::*;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct VectorTable<R> {
        data: Vec<R>,
    }

    impl<R> VectorTable<R> {
        pub fn new(data: Vec<R>) -> Self {
            VectorTable { data }
        }

        pub fn get_mut(&mut self) -> &mut Vec<R> {
            &mut self.data
        }
    }

    impl<R> Table<R> for VectorTable<R> {
        fn get_table(&self) -> Vec<&R> {
            let mut list = vec![];
            for v in &self.data {
                list.push(v);
            }

            list
        }

        fn get_record(&self, pos: usize) -> Option<&R> {
            self.data.get(pos)
        }
    }

    impl<R> Store for VectorTable<R>
        where
            R: Serialize,
    {
        fn store<W>(&mut self, writer: W) -> io::Result<()>
            where
                W: std::io::Write,
        {
            match serde_json::to_writer(writer, &self) {
                Ok(_) => Ok(()),
                Err(e) => Err(io::Error::new(io::ErrorKind::WriteZero, e)),
            }
        }
    }

    impl<R> Load for VectorTable<R>
        where
            R: DeserializeOwned,
    {
        fn load<Re: io::Read>(reader: Re) -> io::Result<Self> {
            match serde_json::from_reader(reader) {
                Ok(data) => Ok(data),
                Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
            }
        }

        // only required for store_mapped_file
        fn load_slice(from: &[u8]) -> io::Result<Self> {
            match serde_json::from_slice(from) {
                Ok(data) => Ok(data),
                Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
            }
        }
    }
}
