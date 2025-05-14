use serde::{Serialize, de::Deserialize};

pub fn post<T, U>(request: &T, endpoint: &str) -> U
where
    T: Serialize,
    U: for<'de> Deserialize<'de>,
{
    //let url = std::env::var("PLAYGROUND_URL").unwrap();

    let serialized = serde_json::to_string(request).unwrap();
    println!("{}", serialized);

    let deserialized: U = serde_json::from_str(&serialized).unwrap();
    deserialized
}

#[cfg(test)]
mod tests {
    use super::post;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestStruct {
        success: bool,
        result: u32,
    }

    #[test]
    fn test() {
        let test = TestStruct {
            success: true,
            result: 4,
        };

        let result: TestStruct = post(&test, "execute");

        assert_eq!(test, result);
    }
}
