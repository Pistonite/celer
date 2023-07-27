//! Hook to get section names from document
//!
//! Note that this is connected to the document store,
//! so make sure components are memoized if needed to avoid
//! rerendering with store updates.

import { useMemo } from "react";
import { useSelector } from "react-redux";
import { documentSelector } from "core/store";

/// Hook to get section names from document
export const useDocSections = (): string[] => {
    const { document, serial } = useSelector(documentSelector);
    // disabling exhaustive deps here because we rely on the serial
    // to know the document is updated
    /* eslint-disable react-hooks/exhaustive-deps*/
    return useMemo(() => {
        if (!document) {
            return [];
        }
        return document.route.map((section) => section.name);
    }, [serial]);
};
