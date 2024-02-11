//! The doc component

import "./Doc.css";
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
import { ExecDoc } from "low/celerc";

import { DocLine } from "./DocLine";
import { DocSection } from "./DocSection";
import { DocLog } from "./utils";
import { DocNoteBlock, DocNoteBlockProps } from "./DocNoteBlock";
import { DocController, initDocController } from "./DocController";
import { Rich } from "./Rich";
import { DocDiagnosticBlock } from "./DocDiagnosticBlock";
import {
    DocContainer,
    DocContentContainer,
    DocDiagnosticContainer,
    DocEnd,
    DocMainContainer,
    DocNoteContainer,
    DocPrefaceContainer,
    DocScroll,
} from "./dom";
import { useDocStyles } from "./styles";

export const DocRoot: React.FC = () => {
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

    if (document.route.length === 0 && document.diagnostics.length === 0) {
        return <HintScreen>This document has no content</HintScreen>;
    }

    return (
        <ErrorBoundary>
            <CachedDocInternal
                serial={serial}
                document={document}
                splitTypes={splitTypes}
                controller={controller}
            />
        </ErrorBoundary>
    );
};

/// Main doc viewer component
///
/// The document is not connected to the store to prevent
/// accidental re-renders. Do not use useSelector in this component.
type DocInternalProps = {
    /// Serial number of the document
    ///
    /// Will only re-render if the serial number changes
    serial: number;
    /// The document to render
    document: ExecDoc;
    splitTypes: string[];
    controller: DocController;
};
const DocInternal: React.FC<DocInternalProps> = ({
    serial,
    document,
    splitTypes,
    controller,
}) => {
    DocLog.info(`rendering document (serial=${serial})`);

    const styles = useDocStyles();

    const flatNotes = document.route.reduce(
        (acc: DocNoteBlockProps[], section, i) => {
            section.lines.forEach((line, j) => {
                if (line.notes.length > 0) {
                    acc.push({
                        sectionIndex: i,
                        lineIndex: j,
                        notes: line.notes,
                    });
                }
            });
            return acc;
        },
        [],
    );

    return (
        <div
            id={DocScroll.id}
            className={styles.docScroll}
            onScroll={() => {
                controller.onScroll();
            }}
            onKeyDown={(e) => {
                // prevent default scrolling behavior
                // because we have our own
                e.preventDefault();
            }}
        >
            <div
                id={DocContainer.id}
                style={{
                    display: "flex",
                    flexDirection: "column",
                    height: "100%",
                    ["--note-min-width" as string]: "48px",
                }}
            >
                <div id={DocDiagnosticContainer.id}>
                    {document.diagnostics.map((diagnostic, i) => (
                        <DocDiagnosticBlock
                            key={i}
                            diagnostic={diagnostic}
                            showCaret={false}
                        />
                    ))}
                </div>
                <div id={DocPrefaceContainer.id}>
                    {document.preface.map((text, i) => (
                        <div key={i} className="docpreface-block">
                            <Rich content={text} size={400} />
                        </div>
                    ))}
                </div>
                <div
                    id={DocContentContainer.id}
                    style={{
                        display: "flex",
                    }}
                >
                    <div
                        id={DocMainContainer.id}
                        style={{
                            minWidth:
                                "min(calc(100% - var(--note-min-width)), 380px)",
                            maxWidth: "380px",
                        }}
                    >
                        {document.route.map(({ name, lines }, i) => (
                            <DocSection index={i} key={i} name={name}>
                                {lines.map((line, j) => {
                                    const counterTag =
                                        line.counterText?.tag || undefined;
                                    const splitType =
                                        counterTag &&
                                        document.project.tags[counterTag]
                                            ?.splitType;
                                    const isSplit =
                                        splitType &&
                                        splitTypes.includes(splitType);
                                    return (
                                        <DocLine
                                            sectionIndex={i}
                                            lineIndex={j}
                                            key={j}
                                            diagnostics={line.diagnostics}
                                            lineColor={line.lineColor}
                                            text={line.text}
                                            iconUrl={
                                                line.icon
                                                    ? document.project.icons[
                                                          line.icon
                                                      ]
                                                    : undefined
                                            }
                                            secondaryText={line.secondaryText}
                                            counterText={line.counterText}
                                            counterType={counterTag}
                                            isBanner={line.isBanner}
                                            isSplit={!!isSplit}
                                            splitType={splitType || undefined}
                                        />
                                    );
                                })}
                            </DocSection>
                        ))}
                    </div>
                    <div
                        id={DocNoteContainer.id}
                        style={{
                            // need this because note blocks have position: absolute
                            position: "relative",
                            flex: 1,
                            minWidth: "var(--note-min-width)",
                            // need for container query on the notes
                            containerType: "size",
                        }}
                    >
                        {flatNotes.map((props, i) => (
                            <DocNoteBlock key={i} {...props} />
                        ))}
                    </div>
                </div>
                <div
                    id={DocEnd.id}
                    style={{
                        flex: 1,
                        textAlign: "center",
                        padding: "32px",
                        color: "#888",
                    }}
                >
                    There's nothing more to see past this point.
                </div>
            </div>
        </div>
    );
};
const CachedDocInternal = memo(
    DocInternal,
    (prev, next) =>
        prev.serial === next.serial && prev.splitTypes === next.splitTypes,
);
