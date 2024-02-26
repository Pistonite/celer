//! The doc component

import React, { memo, useMemo } from "react";
import { useSelector, useStore } from "react-redux";
import { ErrorBoundary, HintScreen, LoadScreen } from "ui/shared";
import { useDocSplitTypes } from "core/doc";
import {
    AppStore,
    documentSelector,
    settingsSelector,
    viewSelector,
} from "core/store";
import { consoleDoc as console } from "low/utils";

import { DocRootProps, DocRoot } from "./components";
import { initDocController } from "./DocController";

export const Doc: React.FC = () => {
    const { stageMode, isEditingLayout, compileInProgress } =
        useSelector(viewSelector);
    const { document, serial } = useSelector(documentSelector);
    const { hideDocWhenResizing } = useSelector(settingsSelector);
    const splitTypes = useDocSplitTypes();

    const store = useStore();
    const controller = useMemo(() => {
        return initDocController(store as AppStore);
    }, [store]);

    if (!document) {
        if (stageMode === "edit" && !compileInProgress) {
            return (
                <HintScreen>
                    Document will be shown here once a project is opened
                </HintScreen>
            );
        }
        return <LoadScreen color="yellow" />;
    }

    if (isEditingLayout && hideDocWhenResizing) {
        // DOM resizing is expensive, so we don't want to do it while editing
        return (
            <HintScreen>
                <p>
                    Document is set to be hidden while the layout is being
                    edited.
                </p>
                <p>You can change this in the settings.</p>
            </HintScreen>
        );
    }

    if (
        document.preface.length === 0 &&
        document.route.length === 0 &&
        document.diagnostics.length === 0
    ) {
        return <HintScreen>This document has no content</HintScreen>;
    }

    return (
        <ErrorBoundary>
            <DocRootCached
                serial={serial}
                document={document}
                splitTypes={splitTypes}
                onScroll={() => controller.onScroll()}
                onRender={() => {
                    // doing this so we can check for excessive re-renders
                    console.info(`rendering document (serial=${serial})`);
                }}
            />
        </ErrorBoundary>
    );
};
const DocRootCached = memo(DocRoot, areDocPropsEqual);

function areDocPropsEqual(prev: DocRootProps, next: DocRootProps) {
    return prev.serial === next.serial && prev.splitTypes === next.splitTypes;
}
