use bytes;

use md5::{self, Digest};

/// function takes data in &str type and calulates md5hash.
/// hexdigest aoutput data from md5 function compute as struct Digest
/// result converts it with format! macro to String
/// data:: &str 
/// return:: result as String
pub fn md5hash(data: &str) -> String {
    let hexdigest = md5::compute(data);
    let result = format!("{:?}", hexdigest);
    return result;
    
}
/// function takes Bytes from bytes crate as argument and calculates md5 hashstring
/// data:: Bytes
/// return:: String
pub fn md5bytes(data:bytes::Bytes) -> String {
    let hexdigest: Digest = md5::compute(data);
    let result = format!("{:?}", hexdigest);
    return result;
}


/// First test because I know not how to put tests in tests folder NOW !
#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use crate::hashtools::md5bytes;

    // TODO: Move tests to Folder tests
    use crate::hashtools::md5hash;

    #[test]
    /// this test calculates md5 value for String 2
    fn md5_two() {
        let result : String = String::from("c81e728d9d4c2f636f067f89cc14862c");
        assert_eq!(result, md5hash(stringify!(2)));
    }

    #[test]
    /// this test calculates md5 value for String 3
    fn md5_three() {
        let result : String = String::from("eccbc87e4b5ce2fe28308fd9f2a7baf3");
        assert_eq!(result, md5hash(stringify!(3)));
    }

    #[test]
    /// this test calculates md5 value for String 3
    fn md5_hello() {
        let result : String = String::from("8b1a9953c4611296a827abf8c47804d7");
        let b = Bytes::from_static(b"Hello");
        assert_eq!(result, md5bytes(b));
    }
}