//! The doc component

import "./Doc.css";
import React from "react";
import { useSelector, useStore } from "react-redux";
import { ErrorBoundary, HintScreen, LoadScreen } from "ui/shared";
import { ExecDoc } from "low/compiler.g";
import { AppStore, documentSelector, viewSelector } from "core/store";

import { DocLine } from "./DocLine";
import { DocSection } from "./DocSection";
import {
    DocContainerId,
    DocLog,
    DocScrollId,
    resolveTag,
    resolveTags,
} from "./utils";
import { DocNoteBlock, DocNoteBlockProps } from "./DocNoteBlock";
import { DocNoteContainerId } from "./updateNotePositions";
import { DocController, initDocController } from "./DocController";

export const DocRoot: React.FC = () => {
    const { isEditingLayout } = useSelector(viewSelector);
    const { document, serial } = useSelector(documentSelector);
    const store = useStore();
    const controller = React.useMemo(() => {
        return initDocController(store as AppStore);
    }, [store]);

    if (!document) {
        return <LoadScreen color="yellow" />;
    }

    if (isEditingLayout) {
        // DOM resizing is expensive, so we don't want to do it while editing
        return <HintScreen message="Document hidden while editing layout" />;
    }
    return (
        <ErrorBoundary>
            <CachedDocInternal
                serial={serial}
                document={document}
                controller={controller}
            />
        </ErrorBoundary>
    );
};

/// Main doc viewer component
type DocInternalProps = {
    /// Serial number of the document
    ///
    /// Will only re-render if the serial number changes
    serial: number;
    /// The document to render
    document: ExecDoc;
    /// The controller
    controller: DocController;
};
const DocInternal: React.FC<DocInternalProps> = ({ document, controller }) => {
    DocLog.info("rendering document");
    const tagMap = document.project.tags;
    const flatNotes = document.route.reduce(
        (acc: DocNoteBlockProps[], section, i) => {
            section.lines.forEach((line, j) => {
                if (line.notes.length > 0) {
                    acc.push({
                        sectionIndex: i,
                        lineIndex: j,
                        notes: line.notes,
                        tagMap,
                    });
                }
            });
            return acc;
        },
        [],
    );

    return (
        <div
            id={DocScrollId}
            onScroll={() => {
                controller.onScroll();
            }}
            onKeyDown={(e) => {
                // prevent default scrolling behavior
                // because we have our own
                e.preventDefault();
            }}
        >
            <div id={DocContainerId}>
                <div id="doc-main">
                    {document.route.map(({ name, lines }, i) => (
                        <DocSection index={i} key={i} name={name}>
                            {lines.map((line, j) => (
                                <DocLine
                                    sectionIndex={i}
                                    lineIndex={j}
                                    key={j}
                                    diagnostics={line.diagnostics}
                                    lineColor={line.lineColor}
                                    text={resolveTags(tagMap, line.text)}
                                    iconUrl={line.icon ? document.project.icons[line.icon] : undefined}
                                    secondaryText={resolveTags(
                                        tagMap,
                                        line.secondaryText,
                                    )}
                                    counterText={
                                        line.counterText
                                            ? resolveTag(
                                                  tagMap,
                                                  line.counterText,
                                              )
                                            : undefined
                                    }
                                    counterType={
                                        line.counterText?.tag || undefined
                                    }
                                />
                            ))}
                        </DocSection>
                    ))}
                    <div id="doc-end">
                        You have reached the end of the document :))
                    </div>
                </div>
                <div id={DocNoteContainerId}>
                    {flatNotes.map((props, i) => (
                        <DocNoteBlock key={i} {...props} />
                    ))}
                </div>
            </div>
        </div>
    );
};
const CachedDocInternal = React.memo(
    DocInternal,
    (prev, next) =>
        prev.serial === next.serial && prev.controller === next.controller,
);
