use {
    crate::types::{Deserialize, Serialize},
    std::{
        fmt::Debug,
        io::{self, Error, ErrorKind},
    },
};

pub fn assert_match<T: Debug + Serialize, U: Debug + Deserialize>(
    actual: &T,
    expected: &U,
) -> io::Result<()> {
    println!("Value to be serialized: {:?}", actual);

    let size = actual.size();
    println!("Expected size of the serialized value: {:?}", size);

    let mut buffer = Vec::<u8>::new();
    actual.serialize(&mut buffer)?;
    println!("Bytes from serialization: {:?}", buffer);

    println!("Size of the serialized value: {:?}", buffer.len());
    if buffer.len() != size {
        return Err(Error::new(ErrorKind::Other, "Mismatch!"));
    }

    let mut slice = buffer.as_slice();
    let replica = U::deserialize(&mut slice)?;
    println!("Value deserialized from those bytes: {:?}", replica);

    if !slice.is_empty() {
        return Err(Error::new(
            ErrorKind::Other,
            "The buffer was not consumed completely!",
        ));
    }

    if format!("{:?}", replica) != format!("{:?}", expected) {
        return Err(Error::new(ErrorKind::Other, "Mismatch!"));
    }

    Ok(())
}

pub fn assert_round_trip<T: Debug + Serialize + Clone, U: Debug + Deserialize + From<T>>(
    value: &T,
) -> io::Result<()> {
    assert_match(value, &U::from(value.clone()))
}
