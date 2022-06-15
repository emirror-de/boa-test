#[macro_use]
extern crate gc;

use boa::{
    class::{Class, ClassBuilder},
    gc::{Finalize, Trace},
    object::{JsObject, ObjectInitializer},
    property::{Attribute, PropertyDescriptor},
    Context,
    JsResult,
    JsString,
    JsValue,
};

#[derive(Debug, Trace, Finalize)]
pub struct Document {}

impl Document {
    pub fn new() -> Self {
        Document {}
    }
    pub fn get_elements_by_tag_name(
        this: &JsValue,
        _: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        if let Some(object) = this.as_object() {
            // If it is we downcast the type to type `Person`.
            if let Some(_person) = object.downcast_ref::<Document>() {
                println!("Done!");
                return Ok(JsValue::undefined());
            }
        }
        context.throw_type_error("'this' is not a Person object")
    }
}

impl Class for Document {
    const NAME: &'static str = "Document";
    const LENGTH: usize = 0;
    const ATTRIBUTES: Attribute = Attribute::all();

    fn constructor(
        this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<Self> {
        Ok(Self {})
    }

    fn init(class: &mut ClassBuilder<'_>) -> JsResult<()> {
        class.method("getElementsByTagName", 1, Self::get_elements_by_tag_name);
        Ok(())
    }
}

fn main() {
    let mut context = Context::new();

    let document = Document::new();
    context.register_global_class::<Document>();
    //context.register_global_property("document", document, Attribute::all());

    //let value = context.eval("document.getElementsByTagName()");
    let value = context.eval(
        "document = new Document(); document.getElementsByTagName('tag_name')",
    );
    println!("{:#?}", value);
}
