use celerc::comp::CompDoc;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use serde_json::Value;
use wasm_bindgen::prelude::*;

mod utils;
use utils::*;

mod resource;


wasm_import!{
    import { ExecDoc } from "low/compiler.g";
}

wasm_from!{CompDoc}
wasm_from!{String}

wasm_api!(
    /// Test
    pub async fn testSomething(x: String) -> number {
        let n: u64 = 9000000000000000000;
        utils::log(&format!("n = {} start", n));
        let n = celerc::test_number(n).await;
        utils::log(&format!("n = {} done", n));
        n
    }
    /// Test converting from compdoc
    pub async fn tryCompileFromCompDoc(comp_doc: CompDoc) -> ExecDoc {
        utils::log("here");
        // let comp_doc = CompDoc::from_wasm(comp_doc)?;
        // let comp_doc: CompDoc = serde_wasm_bindgen::from_value(comp_doc)?;
        utils::log("here1");
        let exec_doc = comp_doc.exec().await;
        utils::log("here2");
        // let serializer = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        exec_doc

        // exec_doc.serialize(&serializer)

        // let r = exec_doc.serialize(&serializer)?;
        //
        // Ok(r)
    }
);

