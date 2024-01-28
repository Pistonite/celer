
import { useMemo } from "react";
import { useSelector } from "react-redux";

import { documentSelector, settingsSelector } from "core/store";

/// Hook to get the disabled plugins for the current document
export const useDocDisabledPlugins = (): string[] => {
    const { document, serial } = useSelector(documentSelector);
    const { disabledPlugins } = useSelector(settingsSelector);

    /* eslint-disable react-hooks/exhaustive-deps*/
    const disabledPluginsForCurrentDoc = useMemo(() => {
        if (!document) {
            return [];
        }
        const title = document.project.title;
        return disabledPlugins[title] || [];
    }, [serial, disabledPlugins]);
    /* eslint-enable react-hooks/exhaustive-deps*/
    
    return disabledPluginsForCurrentDoc;
}
