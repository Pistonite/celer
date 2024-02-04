//! An editor that uses PrismJS for syntax highlighting
import React, { Suspense } from "react";
import { Spinner } from "@fluentui/react-components";

import { PrismEditorProps } from "./types";

const PrismEditorCore = React.lazy(() => import("./PrismEditorCore"));

export const PrismEditor: React.FC<PrismEditorProps> = (props) => {
    return (
        <Suspense fallback={<Spinner />}>
            <PrismEditorCore {...props} />
        </Suspense>
    );
};
