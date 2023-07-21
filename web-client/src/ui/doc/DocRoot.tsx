//! The doc component

import "./Doc.css";
import React, { useEffect } from "react";
import { useSelector } from "react-redux";

import { HintScreen, LoadScreen, useAppStore } from "ui/shared";

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
import { initDocController, DocController } from "./DocController";
import { ErrorBoundary } from "ui/shared/ErrorBoundary";
import { ExecDoc } from "low/compiler";
import { documentSelector, viewSelector } from "core/store";

/// Doc wrapper component that connects to the store
/// The underlying component is cached to avoid unnecessary re-rendering
/// Because document is very expensive to render
type DocRootProps = {
    /// The controller for user actions on the viewer
    controller: DocController,
};
export const DocRoot: React.FC<DocRootProps> = ({ controller }) => {
    const { isEditingLayout } = useSelector(viewSelector);
    const { document, serial } = useSelector(documentSelector);

    if (!document.loaded) {
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
    serial: number,
    /// The document to render
    document: ExecDoc,
    /// The controller for user actions on the viewer
    controller: DocController,
}
const DocInternal: React.FC<DocInternalProps> = ({ controller, document }) => {
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
            tabIndex={0}
            id={DocScrollId}
            onScroll={() => {
                controller.onScroll();
            }}
            onFocus={() => {
                console.log("focus");
            }}
            onKeyDown={(e) => {
                console.log("key up", e.key);
            }}
            onKeyUp={(e) => {
                console.log("key down", e.key);
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
                                    iconUrl={document.project.icons[line.icon]}
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
                                />
                            ))}
                        </DocSection>
                    ))}
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
    (prev, next) => prev.serial === next.serial,
);
