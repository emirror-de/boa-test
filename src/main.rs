use boa::{
    object::{JsObject, ObjectInitializer},
    property::{Attribute, PropertyDescriptor},
    Context,
    JsString,
    JsValue,
};

fn create_document(context: &mut Context) -> JsObject {
    ObjectInitializer::new(context)
        .function(
            |_, _, _| Ok(JsValue::String(JsString::new("tag name"))),
            "getElementsByTagName",
            0,
        )
        .build()
}

fn main() {
    let mut context = Context::new();
    let document = create_document(&mut context);

    context.register_global_property("document", document, Attribute::all());

    let value = context.eval("document.getElementsByTagName()");
    println!("{:#?}", value);
}
