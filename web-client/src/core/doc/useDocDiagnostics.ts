//! Hook for accessing diagnostics for the document
//!
//! Note that this is connected to the document store,
//! so make sure components are memoized if needed to avoid
//! rerendering with store updates.

import { useMemo } from "react";
import { useSelector } from "react-redux";
import { DocDiagnostic } from "low/compiler.g";
import { documentSelector } from "core/store";

import { removeTags } from "./utils";

/// Data for one diagnostic
export type DiagnosticSection = {
    sectionName: string;
    diagnostics: DiagnosticWithLocation[];
};
export type DiagnosticWithLocation = DocDiagnostic & {
    /// Text of the line that has the diagnostic
    lineText: string;
    /// Section number
    sectionIndex: number;
    /// Line number
    lineIndex: number;
};

/// Hook to get all diagnostics in the document
export const useDocDiagnostics = (): DiagnosticSection[] => {
    const { document, serial } = useSelector(documentSelector);
    // disabling exhaustive deps here because we rely on the serial
    // to know the document is updated
    /* eslint-disable react-hooks/exhaustive-deps*/
    const diagnostics = useMemo(() => {
        const output: DiagnosticSection[] = [];
        if (!document) {
            return output;
        }
        document.route.forEach((section, i) => {
            const diagnosticSection: DiagnosticSection = {
                sectionName: section.name,
                diagnostics: [],
            };
            section.lines.forEach((line, j) => {
                line.diagnostics.forEach((diag) => {
                    diagnosticSection.diagnostics.push({
                        ...diag,
                        lineText: removeTags(line.text),
                        sectionIndex: i,
                        lineIndex: j,
                    });
                });
            });
            if (diagnosticSection.diagnostics.length > 0) {
                output.push(diagnosticSection);
            }
        });
        return output;
    }, [serial]);

    return diagnostics;
};
