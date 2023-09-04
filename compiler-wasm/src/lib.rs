use celerc::comp::CompDoc;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

macro_rules! js_await {
    ($($call:tt)*) => {
        {
            let promise_result = $($call)*;
            let promise = promise_result?;
            let promise = js_sys::Promise::resolve(&promise);
            wasm_bindgen_futures::JsFuture::from(promise).await?
        }
    };
}

macro_rules! wasm_api {
    (
        import {
            $($import:ty),*
        }
        $(
            $(#[doc = $doc:literal])*
            export async fn $name:ident($($arg:ident: $arg_ty:ty),*) -> $ret_ty:ty $body:block
        )*
    ) => {
        // define typescript doc and function

        pub fn generate_d_ts() -> String {
            let mut d_ts = String::new();
            d_ts.push_str("import { ");
            $(
                d_ts.push_str(stringify!($import));
                d_ts.push_str(", ");
            )*
            d_ts.push_str(" } from \"low/compiler.g\";\n\n");
            $(
                d_ts.push_str(concat!(
                    $(
                        "///", $doc, "\n",
                    )*
                    "export function ", stringify!($name), "(", $(stringify!($arg), ": ", stringify!($arg_ty), ",")*,
                    "): Promise<", stringify!($ret_ty), ">;\n\n"
                ));
            )*
            d_ts
        }

        $(
            // define docs
            $(#[doc = $doc])*
            #[allow(non_snake_case)]
            #[wasm_bindgen(js_name = $name, skip_typescript)]
            pub async fn $name($($arg: JsValue),*) -> Result<JsValue, JsValue> $body
        )*
    }
}

wasm_api!(
    import { ExecDoc }

    /// Test converting from compdoc
    export async fn tryCompileFromCompDoc(comp_doc: any) -> ExecDoc {
        log("here");
        let comp_doc: CompDoc = serde_wasm_bindgen::from_value(comp_doc)?;
        log("here1");
        let exec_doc = comp_doc.exec().await;
        log("here2");
        let serializer = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        let r = exec_doc.serialize(&serializer)?;

        Ok(r)
    }
);
