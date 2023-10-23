// TODO #78: async_for no longer needed in the future

/// Transform all [`CompLine`] in a document
macro_rules! for_each_line {
    ($line:ident in $comp_doc:ident $fun:block) => {
        let _ = $crate::util::async_for!(section in $comp_doc.route.iter_mut(), {
            let lines = std::mem::take(&mut section.lines);
            let _ = $crate::util::async_for!(mut $line in lines.into_iter(), {
                let l = $fun;
                section.lines.push(l);
            });
        });
    };
}
pub(crate) use for_each_line;

macro_rules! for_each_rich_text_except_counter {
    ($t:ident in $comp_line:ident $fun:block) => {
        let _ = $crate::util::async_for!($t in $comp_line.text.iter_mut(), {
            $fun;
        });
        let _ = $crate::util::async_for!($t in $comp_line.secondary_text.iter_mut(), {
            $fun;
        });
        let _ = $crate::util::async_for!(note in $comp_line.notes.iter_mut(), {
            match note { 
                $crate::types::DocNote::Text { content} => {
                    let _ = $crate::util::async_for!($t in content.iter_mut(), {
                        $fun;
                    });
                }
                _ => {}
            }
        });
        if let Some(v) = $comp_line.split_name.as_mut() {
            let _ = $crate::util::async_for!($t in v.iter_mut(), {
                $fun;
            });
        }
    };
}
pub(crate) use for_each_rich_text_except_counter;
