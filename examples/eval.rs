use quick_js::{Context, JsValue};
use std::collections::HashMap;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

type JSObject = quick_js::JsValue::Object(HashMap<String, JsValue>);

impl From<JSObject> for User {
    fn from(o: JSObject) -> Self {
        User {
            username: o["username"],
            email: o["email"],
            sign_in_count: o["sign_in_count"],
            active: o["active"]
        }
    }
}

pub fn main() {
    let context = Context::new().unwrap();

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    context.set_global("User1", user1);

    // println!("{:?}", context.eval("User1").unwrap());

    let value = context.eval("1 + 2").unwrap();
    println!("js: 1 + 2 = {:?}", value);

    let value = context
        .eval_as::<String>(" var x = 100 + 250; x.toString() ")
        .unwrap();
    println!("{:?}", value);
    // assert_eq!(&value, "350");

    context
        .add_callback("myCallback", |a: i32, b: i32| a + b * b)
        .unwrap();

    let value = context
        .eval(
            r#"
       var x = myCallback(10, 20);
       x;
"#,
        )
        .unwrap();
    println!("js: callback = {:?}", value);
}
