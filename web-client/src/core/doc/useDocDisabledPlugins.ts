import { useMemo } from "react";
import { useSelector } from "react-redux";

import { documentSelector, settingsSelector } from "core/store";

import type { PluginMetadata } from "low/celerc";

// /// Hook to get the disabled plugins for the current document
// export const useDocDisabledPlugins = (): string[] => {
//     const { document, serial } = useSelector(documentSelector);
//     const { disabledPlugins } = useSelector(settingsSelector);
//
//     /* eslint-disable react-hooks/exhaustive-deps*/
//     const disabledPluginsForCurrentDoc = useMemo(() => {
//         if (!document) {
//             return [];
//         }
//         const title = document.project.title;
//         return disabledPlugins[title] || [];
//     }, [serial, disabledPlugins]);
//     /* eslint-enable react-hooks/exhaustive-deps*/
//
//     return disabledPluginsForCurrentDoc;
// };

/// Hook to get the disabled plugins for the current document
export const useDocPluginMetadata = (): PluginMetadata[] => {
    const { document, serial } = useSelector(documentSelector);
    const { pluginMetadatas } = useSelector(settingsSelector);

    /* eslint-disable react-hooks/exhaustive-deps*/
    return useMemo(() => {
        if (!document) {
            return [];
        }
        const title = document.project.title;
        return pluginMetadatas[title] || [];
    }, [serial, pluginMetadatas]);
    /* eslint-enable react-hooks/exhaustive-deps*/
};
