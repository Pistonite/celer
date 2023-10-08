import { DocPoorText } from "low/compiler.g";
import { Poor } from "./Poor";

export type DocPrefaceProps = {
    text: DocPoorText[];
};

export const DocPreface: React.FC<DocPrefaceProps> = ({ text }) => {
    return (
        <div className="doc-preface-block">
            <Poor content={text} textProps={{ size: 400 }} />
        </div>
    );
};
