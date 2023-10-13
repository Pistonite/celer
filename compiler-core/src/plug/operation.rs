use futures::Future;

use crate::types::DocRichText;
use crate::comp::{CompDoc, CompLine};
use crate::util::async_for;

pub async fn for_all_preface_lines<F>(comp_doc: &mut CompDoc, fun: F)
where
    F: Fn(&mut Vec<DocRichText>),
{
    // TODO #78: async_for no longer needed in the future
    let _ = async_for!(t in comp_doc.preface.iter_mut(), {
        fun(t);
    });
}

/// Transform all [`CompLine`] in a document with function F
pub async fn for_all_lines<Func, Fut>(comp_doc: &mut CompDoc, fun: Func)
where
    Func: Fn(CompLine) -> Fut,
    Fut: Future<Output = CompLine>,
{
    // TODO #78: async_for no longer needed in the future
    let _ = async_for!(section in comp_doc.route.iter_mut(), {
        let lines = std::mem::take(&mut section.lines);
        let _ = async_for!(line in lines.into_iter(), {
            section.lines.push(fun(line).await);
        });
    });
}

/// Transform all rich text in a line with function F
pub async fn for_all_rich_text<F>(comp_line: &mut CompLine, fun: F)
where
    F: Fn(&mut DocRichText),
{
    // TODO #78: async_for no longer needed in the future
    let _ = async_for!(t in comp_line.text.iter_mut(), {
        fun(t);
    });
    let _ = async_for!(t in comp_line.secondary_text.iter_mut(), {
        fun(t);
    });
    if let Some(t) = comp_line.counter_text.as_mut() {
        fun(t);
    }
    if let Some(v) = comp_line.split_name.as_mut() {
        let _ = async_for!(t in v.iter_mut(), {
            fun(t);
        });
    }
}
