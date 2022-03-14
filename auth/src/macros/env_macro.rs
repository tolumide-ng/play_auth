/// ------->>>>>>      https://stackoverflow.com/questions/58514822/print-all-struct-fields-in-rust
/// 
/// first implement a trait type of macro which is wrapped around the struct on declaration
/// e.g 
/// MyMacro! {
///     struct MyStruct {}
/// }
/// 
/// macro_rules! MyMacro {} implements a sort of trait for the structs it's declared with, basically, it should
/// memoize the type of each field of the struct
/// 
/// 
/// The other/second macro just derives the real values from the environment variables for the struct it's wrapped
/// around + parses such values obtained from the env variable into the desired type
/// 
/// e.g let abc = MyStruct::new(a, b, c);
/// let real_values = get_values!(abc);
/// 




// #[macro_export]
// macro_rules! as_expr { ($e:expr) => {$e} }


// use crate::as_expr;

// #[macro_export]
// macro_rules! foo {
//     ($($tts:tt)*) => {
//         as_expr!($($tts)*)
//     };
// }

#[macro_export]
macro_rules! as_talk { ($a:expr) => {$a} }

#[macro_export]
macro_rules! parse_env {
    ($field:ident:$type:ty) => {
        let ab = stringify!($field).to_string();
        let cd = stringify!($type).to_string();
        println!("Got called!! ((((((((((((()))))))))))))))))) {:#?} ---------------- {:#?} ((((((((((((())))))))))))))))))", ab, cd)
    };
    ($vis:vis $struct:ident {$( $field:ident:$type:ty ),*,}) => (
        // $(  )*
        $( parse_env!($field:$type); ) *
    );
}

#[macro_export]
macro_rules! speak {
    ($field:ident:$type:ty) => (
        // println!(concat!(stringify!($type), " = {:?}"), $type);
        // $field;
        let ab = stringify!($field).to_string();
        println!("Got called!! ((((((((((((()))))))))))))))))) {:#?}", ab)
    )
}


