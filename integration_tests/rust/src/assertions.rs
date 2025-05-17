use {
    crate::types::{Deserialize, Serialize},
    std::{
        fmt::Debug,
        fs::{OpenOptions, remove_file},
        io::{self, Error, Write},
        mem::drop,
    },
};

const OMNIFILE_PATH: &str = "/tmp/omnifile-rust";

pub fn start() {
    drop(remove_file(OMNIFILE_PATH));
}

pub fn assert_match<T: Debug + Serialize, U: Debug + Deserialize>(
    actual: &T,
    expected: &U,
) -> io::Result<()> {
    println!("Message to be serialized: {actual:?}");

    let size = actual.size();
    println!("Expected size of the serialized value: {size:?}");

    let mut buffer = Vec::<u8>::new();
    actual.serialize(&mut buffer)?;
    println!("Bytes from serialization: {buffer:?}");
    println!("Size of the serialized value: {:?}", buffer.len());
    if buffer.len() != size {
        return Err(Error::other("Mismatch!"));
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(OMNIFILE_PATH)
        .unwrap();
    file.write_all(&buffer).unwrap();

    let replica = U::deserialize(buffer.as_slice())?;
    println!("Message deserialized from those bytes: {replica:?}");

    if format!("{replica:?}") != format!("{expected:?}") {
        return Err(Error::other("Mismatch!"));
    }

    Ok(())
}

pub fn assert_round_trip<T: Debug + Serialize + Clone, U: Debug + Deserialize + From<T>>(
    value: &T,
) -> io::Result<()> {
    assert_match(value, &U::from(value.clone()))
}
