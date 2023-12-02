import { useMemo } from "react";
import { useSelector } from "react-redux";

import { documentSelector, settingsSelector } from "core/store";
import { getDefaultSplitTypes } from "./utils";

/// Get the current split types as configured in the settings
export const useDocSplitTypes = (): string[] => {
    const { splitTypes } = useSelector(settingsSelector);
    const { serial, document } = useSelector(documentSelector);
    /* eslint-disable react-hooks/exhaustive-deps*/
    const currentSplitTypes = useMemo(() => {
        if (splitTypes) {
            return splitTypes;
        }
        if (document) {
            return getDefaultSplitTypes(document);
        }
        return [];
    }, [serial, splitTypes]);
    /* eslint-enable react-hooks/exhaustive-deps*/
    return currentSplitTypes;
};
