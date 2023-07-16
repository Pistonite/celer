//! The doc component

import "./Doc.css";
import React from "react";
import { useSelector } from "react-redux";

import { HintScreen, LoadScreen } from "ui/shared";
import { DocumentStore, documentSelector, viewSelector } from "data/store";

import { DocLine } from "./DocLine";
import { DocSection } from "./DocSection";
import { DocContainerId, DocLog, DocScrollId, resolveTag, resolveTags } from "./util";
import { DocNoteBlock, DocNoteBlockProps } from "./DocNoteBlock";
import { DocNoteContainerId } from "./updateNotePositions";
import { initDocController } from "./DocController";
import { ErrorBoundary } from "ui/shared/ErrorBoundary";

const DocController = initDocController();

/// Doc wrapper component that connects to the store
/// The underlying component is cached to avoid unnecessary re-rendering
/// Because document is very expensive to render
export const Doc: React.FC = () => {
    const { isEditingLayout } = useSelector(viewSelector);
    const documentStore = useSelector(documentSelector);
    if (isEditingLayout) {
        // DOM resizing is expensive, so we don't want to do it while editing
        return <HintScreen message="Document hidden while editing layout" />;
    }
    return (
        <ErrorBoundary>
            <CachedDocInternal {...documentStore} />
        </ErrorBoundary>
    )
};

/// Main doc viewer component
const DocInternal: React.FC<DocumentStore> = ({ document }) => {
    if (!document.loaded) {
        return <LoadScreen color="yellow" />;
    }
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
                DocController.onScroll();
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
