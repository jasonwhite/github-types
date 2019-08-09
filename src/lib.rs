// Copyright (c) 2019 Jason White
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// macro usage isn't detected properly
#[allow(unused_macros)] 
macro_rules! test_generator {
    (
    TypeName: $type_name: ident;
    TestData: $test_data: expr;
    $(assert!($field_name: ident == $field_value: expr);)*
    ) => {
        let json_value: $type_name = match ::serde_json::from_str($test_data) {
            Ok(json_value) => json_value,
            Err(err) => panic!("{:?}", err),
        };
        $(
            assert_eq!(json_value.$field_name, $field_value);
        )*
    };
}

mod app;
mod checks;
mod datetime;
mod events;
mod oid;
mod previews;
mod repo;
mod user;

pub use app::*;
pub use checks::*;
pub use datetime::*;
pub use events::*;
pub use oid::*;
pub use previews::*;
pub use repo::*;
pub use user::*;
