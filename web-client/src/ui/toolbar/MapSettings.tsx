//! Map tab of the settings dialog

import {
    Dropdown,
    Field,
    Option,
    Switch,
    Label,
    Slider,
    SliderProps,
    DropdownProps,
} from "@fluentui/react-components";
import { useSelector } from "react-redux";
import { settingsActions, settingsSelector } from "core/store";
import { LayerMode, SectionMode, VisualSize } from "core/map";
import { useActions } from "low/store";

import { SettingsSection } from "./SettingsSection";

export const MapSettings: React.FC = () => {
    const {
        iconSectionMode,
        iconLayerMode,
        lineSectionMode,
        lineLayerMode,
        markerSectionMode,
        markerLayerMode,
        fadeNonCurrentLayerIcons,
        fadeNonCurrentLayerLines,
        fadeNonCurrentLayerMarkers,
        primaryIconSize,
        secondaryIconSize,
        otherIconSize,
        lineSize,
        markerSize,
        arrowSize,
        arrowFrequency,
    } = useSelector(settingsSelector);
    const {
        setIconSectionMode,
        setIconLayerMode,
        setLineSectionMode,
        setLineLayerMode,
        setMarkerSectionMode,
        setMarkerLayerMode,
        setFadeNonCurrentLayerIcons,
        setFadeNonCurrentLayerLines,
        setFadeNonCurrentLayerMarkers,
        setPrimaryIconSize,
        setSecondaryIconSize,
        setOtherIconSize,
        setLineSize,
        setMarkerSize,
        setArrowSize,
        setArrowFrequency,
    } = useActions(settingsActions);
    return (
        <>
            <SettingsSection title="Icons">
                <SectionModeSelector
                    category="icons"
                    value={iconSectionMode}
                    setValue={setIconSectionMode}
                />
                <LayerModeSelector
                    category="icons"
                    selectedLayerMode={iconLayerMode}
                    setValue={setIconLayerMode}
                    disabled={iconSectionMode === SectionMode.None}
                />
                <Switch
                    label="Fade icons not on current layer"
                    checked={!!fadeNonCurrentLayerIcons}
                    onChange={(_, data) =>
                        setFadeNonCurrentLayerIcons(data.checked)
                    }
                    disabled={
                        iconSectionMode === SectionMode.None ||
                        iconLayerMode === LayerMode.CurrentOnly
                    }
                />
                <VisualSizeSelector
                    allowHidden
                    label="Primary icon size"
                    hint="Size of primary icons. Which icons are primary depends on the route."
                    strings={GraphicSizeStrings}
                    value={primaryIconSize}
                    setValue={setPrimaryIconSize}
                    disabled={iconSectionMode === SectionMode.None}
                />
                <VisualSizeSelector
                    allowHidden
                    label="Secondary icon size"
                    hint="Size of secondary icons. Which icons are secondary depends on the route."
                    strings={GraphicSizeStrings}
                    value={secondaryIconSize}
                    setValue={setSecondaryIconSize}
                    disabled={iconSectionMode === SectionMode.None}
                />
                <VisualSizeSelector
                    allowHidden
                    label="Other icon size"
                    hint="Size of all other icons"
                    strings={GraphicSizeStrings}
                    value={otherIconSize}
                    setValue={setOtherIconSize}
                    disabled={iconSectionMode === SectionMode.None}
                />
            </SettingsSection>
            <SettingsSection title="Lines">
                <SectionModeSelector
                    category="lines"
                    value={lineSectionMode}
                    setValue={setLineSectionMode}
                />
                <LayerModeSelector
                    category="lines"
                    selectedLayerMode={lineLayerMode}
                    setValue={setLineLayerMode}
                    disabled={lineSectionMode === SectionMode.None}
                />
                <Switch
                    label="Fade lines not on current layer"
                    checked={!!fadeNonCurrentLayerLines}
                    onChange={(_, data) =>
                        setFadeNonCurrentLayerLines(data.checked)
                    }
                    disabled={
                        lineSectionMode === SectionMode.None ||
                        lineLayerMode === LayerMode.CurrentOnly
                    }
                />
                <VisualSizeSelector
                    allowHidden
                    label="Line thickness"
                    hint="Choose how thick the lines should be"
                    strings={LineSizeStrings}
                    value={lineSize}
                    setValue={setLineSize}
                    disabled={lineSectionMode === SectionMode.None}
                />
                <VisualSizeSelector
                    allowHidden
                    label="Arrow size"
                    hint="Choose how large the arrows on the lines should be. Turning arrows off may improve lag."
                    strings={GraphicSizeStrings}
                    value={arrowSize}
                    setValue={setArrowSize}
                    disabled={lineSectionMode === SectionMode.None}
                />
                <VisualSizeSelector
                    allowHidden={false}
                    label="Arrow spacing"
                    hint="Choose how far apart the arrows should be."
                    strings={ArrowFrequencyStrings}
                    value={arrowFrequency}
                    setValue={setArrowFrequency}
                    disabled={
                        lineSectionMode === SectionMode.None ||
                        arrowSize === VisualSize.Hidden
                    }
                />
            </SettingsSection>
            <SettingsSection title="Markers">
                <SectionModeSelector
                    category="markers"
                    value={markerSectionMode}
                    setValue={setMarkerSectionMode}
                />
                <LayerModeSelector
                    category="markers"
                    selectedLayerMode={markerLayerMode}
                    setValue={setMarkerLayerMode}
                    disabled={markerSectionMode === SectionMode.None}
                />
                <Switch
                    label="Fade markers not on current layer"
                    checked={!!fadeNonCurrentLayerMarkers}
                    onChange={(_, data) =>
                        setFadeNonCurrentLayerMarkers(data.checked)
                    }
                    disabled={
                        markerSectionMode === SectionMode.None ||
                        markerLayerMode === LayerMode.CurrentOnly
                    }
                />
                <VisualSizeSelector
                    allowHidden
                    label="Marker size"
                    hint="Choose how big the circle markers should be"
                    strings={GraphicSizeStrings}
                    value={markerSize}
                    setValue={setMarkerSize}
                    disabled={markerSectionMode === SectionMode.None}
                />
            </SettingsSection>
        </>
    );
};

/// Props for SectionModeSelector
type SectionModeSelectorProps = {
    /// A category word to use in the hint
    category: string;
    /// Selected value
    value: SectionMode;
    /// Callback to set value
    setValue: (value: SectionMode) => void;
};

/// Text displayed in the section mode selector dropdown
const SectionModeTexts = {
    [SectionMode.All]: "Show all sections",
    [SectionMode.CurrentHighlight]:
        "Show all sections, highlight current section",
    [SectionMode.Current]: "Show only current section",
    [SectionMode.None]: "Show nothing",
};

/// Section mode selector component
const SectionModeSelector: React.FC<SectionModeSelectorProps> = ({
    category,
    value,
    setValue,
}) => {
    return (
        <Field
            label="Section mode"
            hint={`Choose how the map displays the ${category} when scrolling through different sections in the route`}
        >
            <Dropdown
                value={SectionModeTexts[value]}
                selectedOptions={[value]}
                onOptionSelect={(_, data) => {
                    setValue(data.selectedOptions[0] as SectionMode);
                }}
            >
                <Option
                    text={SectionModeTexts[SectionMode.All]}
                    value={SectionMode.All}
                >
                    All
                </Option>
                <Option
                    text={SectionModeTexts[SectionMode.CurrentHighlight]}
                    value={SectionMode.CurrentHighlight}
                >
                    All (highlight current)
                </Option>
                <Option
                    text={SectionModeTexts[SectionMode.Current]}
                    value={SectionMode.Current}
                >
                    Current section
                </Option>
                <Option
                    text={SectionModeTexts[SectionMode.None]}
                    value={SectionMode.None}
                >
                    None
                </Option>
            </Dropdown>
        </Field>
    );
};

/// Props for LayerModeSelector
type LayerModeSelectorProps = {
    /// A category word to use in the hint
    category: string;
    /// Selected value
    selectedLayerMode: LayerMode;
    /// Callback to set value
    setValue: (value: LayerMode) => void;
};

/// Text displayed in the layer mode selector dropdown
const LayerModeTexts = {
    [LayerMode.CurrentOnly]: "Show only current layer",
    [LayerMode.CurrentAndAdjacent]: "Show current and adjacent layers",
    [LayerMode.All]: "Show all layers",
};

/// Layer mode selector component
const LayerModeSelector: React.FC<DropdownProps & LayerModeSelectorProps> = ({
    category,
    selectedLayerMode,
    setValue,
    ...props
}) => {
    return (
        <Field
            label="Layer mode"
            hint={`Choose how the map presents the ${category} on different layers`}
        >
            <Dropdown
                {...props}
                value={LayerModeTexts[selectedLayerMode]}
                selectedOptions={[selectedLayerMode]}
                onOptionSelect={(_, data) => {
                    setValue(data.selectedOptions[0] as LayerMode);
                }}
            >
                <Option
                    text={LayerModeTexts[LayerMode.CurrentOnly]}
                    value={LayerMode.CurrentOnly}
                >
                    Current layer only
                </Option>
                <Option
                    text={LayerModeTexts[LayerMode.CurrentAndAdjacent]}
                    value={LayerMode.CurrentAndAdjacent}
                >
                    Current and adjacent layers
                </Option>
                <Option
                    text={LayerModeTexts[LayerMode.All]}
                    value={LayerMode.All}
                >
                    All layers
                </Option>
            </Dropdown>
        </Field>
    );
};

/// Props for the VisualSizeSelector
type VisualSizeSelectorProps = {
    /// Label for the selector
    label: string;
    /// Hint for the selector
    hint: string;
    /// UI strings for the size value
    strings: Record<VisualSize, string>;
    /// Selected value
    value: VisualSize;
    /// Callback to set value
    setValue: (value: VisualSize) => void;
    /// If the value corresponding to `Hidden` should be enabled
    allowHidden: boolean;
};

/// VisualSizeSelector component
const VisualSizeSelector: React.FC<SliderProps & VisualSizeSelectorProps> = ({
    allowHidden,
    label,
    hint,
    strings,
    value,
    setValue,
    ...props
}) => {
    return (
        <Field label={label} hint={hint}>
            <div className="settings-value-slider">
                <Slider
                    {...props}
                    min={allowHidden ? VisualSize.Hidden : VisualSize.Small}
                    max={VisualSize.Large}
                    step={1}
                    value={value}
                    onChange={(_, data) => setValue(data.value as VisualSize)}
                />
                <Label>{strings[value]}</Label>
            </div>
        </Field>
    );
};

/// Strings for icon and marker sizes
const GraphicSizeStrings = {
    [VisualSize.Hidden]: "Hidden",
    [VisualSize.Small]: "Small",
    [VisualSize.Regular]: "Regular",
    [VisualSize.Large]: "Large",
};

/// Strings for line thickness
const LineSizeStrings = {
    [VisualSize.Hidden]: "None",
    [VisualSize.Small]: "Thin",
    [VisualSize.Regular]: "Regular",
    [VisualSize.Large]: "Thick",
};

/// Strings for arrow frequency
const ArrowFrequencyStrings = {
    [VisualSize.Hidden]: "",
    [VisualSize.Small]: "Dense (Slower)",
    [VisualSize.Regular]: "Regular",
    [VisualSize.Large]: "Sparse (Faster)",
};
