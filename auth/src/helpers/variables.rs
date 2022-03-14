use rocket::futures::TryFutureExt;
use serde::{de::Error, Deserialize, Deserializer};
use std::fmt::Display;
use std::str::FromStr;


pub fn with_expand_envs<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de> + std::fmt::Debug,
    <T as FromStr>::Err: Display,
{

#[derive(Deserialize, Debug)]
    #[serde(untagged)]
    enum StringOrAnything<T> {
        String(String),
        Anything(T),
    }
    use dotenv::dotenv;
    dotenv().ok();
    // for (key, value) in std::env::vars() {
    //     println!("{}: {}", key, value);
    // }

    let ab = StringOrAnything::<T>::deserialize(deserializer)?;
    // println!("**************** \n\n {:#?} \n\n ****************", ab);
    match ab {
        StringOrAnything::String(s) => {
            // println!("THE VALUEEEEEEE OF S--------------------------------- {:#?}", s);
            match shellexpand::env(&s) {
                Ok(value) => {
                    // println!("THE VALUE!!!!!!!!!!!!!!!!!!!! {:#?}", value);
                    return value.parse::<T>().map_err(Error::custom)
                },
                Err(err) => {
                    // println!("AN ERROR>>>>>>>>>>>>>>>>> {:#?}", err);
                    return Err(Error::custom(err))
                },
            }
        },
        StringOrAnything::Anything(anything) => {
            // println!("ANYTHINGANYTHINGANYTHINGANYTHINGANYTHINGANYTHIN");
            return Ok(anything)
        }
    };
}