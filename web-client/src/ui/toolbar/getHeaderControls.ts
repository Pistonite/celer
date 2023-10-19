import { StageMode } from "core/stage";

import { HeaderControlList } from "./util";
import { SwitchToolbarLocation } from "./SwitchToolbarLocation";
import { SwitchLayout } from "./SwitchLayout";
import { SwitchMapLayer } from "./SwitchMapLayer";
import { OpenSettings } from "./OpenSettings";
import { ZoomIn, ZoomOut } from "./Zoom";
import { ViewDiagnostics } from "./ViewDiagnostics";
import { SelectSection } from "./SelectSection";
import { CloseProject } from "./CloseProject";
import { SyncProject } from "./SyncProject";
import { SaveProject } from "./SaveProject";
import { CompileProject } from "./CompileProject";
import { OpenDocs } from "./OpenDocs";

/// Header controls.
///
/// The controls are defined in groups.
/// Each control is a ToolbarControl that defines its apperances in the toolbar and in the overflow menu
export const getHeaderControls = (mode: StageMode): HeaderControlList => {
    return [
        // UI Controls
        {
            priority: 30,
            controls: [SwitchLayout, SwitchToolbarLocation],
        },
        // Doc Controls
        {
            priority: 40,
            controls: [SelectSection],
        },
        // Map Controls
        {
            priority: 20,
            controls: [SwitchMapLayer, ZoomIn, ZoomOut],
        },
        // Diagnostic/editor
        {
            // make this section hide last in edit mode
            priority: mode === "edit" ? 89 : 39,
            controls: [
                ViewDiagnostics,
                ...(mode === "edit"
                    ? [CompileProject, SaveProject, SyncProject, CloseProject]
                    : []),
            ],
        },
        // Misc
        {
            priority: 10,
            controls: [OpenSettings, OpenDocs],
        },
    ];
};
