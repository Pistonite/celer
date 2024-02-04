/// Props for the PrismEditor
///
/// This is defined separately because the editor is lazy-loaded
export type PrismEditorProps = {
    language: PrismEditorLanguage;
    value: string;
    setValue: (value: string) => void;
}

export type PrismEditorLanguage = "markup" | "css" | "clike" | "javascript" | "yaml";

