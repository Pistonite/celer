import type { EditorMode } from "core/store";
import type { StageMode } from "core/stage";

import type { HeaderControlList, ToolbarControl } from "./util";
import { SwitchToolbarLocation } from "./SwitchToolbarLocation";
import { SwitchLayout } from "./SwitchLayout";
import { SwitchMapLayer } from "./SwitchMapLayer";
import { OpenSettings } from "./OpenSettings";
import { ZoomIn, ZoomOut } from "./Zoom";
import { ViewDiagnostics } from "./ViewDiagnostics";
import { SelectSection } from "./SelectSection";
import { OpenCloseProject } from "./OpenCloseProject";
import { SyncProject } from "./SyncProject";
import { SaveProject } from "./SaveProject";
import { ReloadDocument } from "./ReloadDocument";
import { OpenDocs } from "./OpenDocs";
import { Export } from "./Export";

/// Header controls.
///
/// The controls are defined in groups.
/// Each control is a ToolbarControl that defines its apperances in the toolbar and in the overflow menu
export const getHeaderControls = (
    mode: StageMode,
    editorMode: EditorMode,
): HeaderControlList => {
    return [
        // UI Controls
        {
            priority: 30,
            controls: [SwitchLayout, SwitchToolbarLocation],
        },
        // Doc Controls
        {
            priority: 40,
            controls: [
                ...(mode === "view" ? [ReloadDocument] : []),
                SelectSection,
                ViewDiagnostics,
                Export,
            ],
        },
        // Map Controls
        {
            priority: 20,
            controls: [SwitchMapLayer, ZoomIn, ZoomOut],
        },
        // Editor
        ...(mode !== "edit"
            ? []
            : [
                  {
                      priority: 89,
                      controls: getEditorControls(editorMode),
                  },
              ]),
        // Misc
        {
            priority: 10,
            controls: [OpenSettings, OpenDocs],
        },
    ];
};

const getEditorControls = (editorMode: EditorMode): ToolbarControl[] => {
    if (editorMode === "web") {
        return [ReloadDocument, SaveProject, SyncProject, OpenCloseProject];
    }
    return [ReloadDocument, OpenCloseProject];
};
