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
import {
    DocContainer,
    DocContentContainer,
    DocLog,
    DocScroll,
    DocNoteContainer,
} from "./utils";
import { DocNoteBlock, DocNoteBlockProps } from "./DocNoteBlock";
import { DocController, initDocController } from "./DocController";
import { Rich } from "./Rich";

export const DocRoot: React.FC = () => {
    const { stageMode, isEditingLayout, compileInProgress } =
        useSelector(viewSelector);
    const { document, serial } = useSelector(documentSelector);
    const store = useStore();
    const controller = useMemo(() => {
        return initDocController(store as AppStore);
    }, [store]);
    const { hideDocWhenResizing } = useSelector(settingsSelector);

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

    const splitTypes = useDocSplitTypes();

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
            style={{
                height: "100%",
                // make document scrollable
                overflowY: "auto",
                // Hide the horizontal scrollbar which could show during layout operations
                overflowX: "hidden",
                scrollBehavior: "smooth",
            }}
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
                <div id="docpreface-container">
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
                        id="doc-main"
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
                    id="doc-end"
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
        prev.serial === next.serial && prev.controller === next.controller,
);
