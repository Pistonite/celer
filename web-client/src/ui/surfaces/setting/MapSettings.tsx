//! Map tab of the settings dialog

import { Dropdown, Field, Option, Switch, Label, Slider } from "@fluentui/react-components"
import { LayerMode, SectionMode, VisualSize, settingsActions, settingsSelector, useActions } from "data/store";
import { useSelector } from "react-redux";
import { SettingsSection } from "./SettingsSection";

export const MapSettings: React.FC = () => {
    const { iconSectionMode, iconLayerMode, lineSectionMode, lineLayerMode, markerSectionMode, markerLayerMode
        ,
        fadeNonCurrentLayerIcons, fadeNonCurrentLayerLines, fadeNonCurrentLayerMarkers,
        primaryIconSize, secondaryIconSize, otherIconSize, lineSize, markerSize

    } = useSelector(settingsSelector);
    const { setIconSectionMode, setIconLayerMode, setLineSectionMode, setLineLayerMode,
        setMarkerSectionMode, setMarkerLayerMode
        ,
        setFadeNonCurrentLayerIcons, setFadeNonCurrentLayerLines, setFadeNonCurrentLayerMarkers,
        setPrimaryIconSize, setSecondaryIconSize, setOtherIconSize, setLineSize, setMarkerSize
    } = useActions(settingsActions);
    return (
        <>
            <SettingsSection title="Icons">
                <SectionModeSelector value={iconSectionMode} setValue={setIconSectionMode} />
                <LayerModeSelector value={iconLayerMode} setValue={setIconLayerMode} />
                <Switch label="Fade icons not on current layer"
                    checked={!!fadeNonCurrentLayerIcons}
                    onChange={(_, data) => setFadeNonCurrentLayerIcons(data.checked)}
                    disabled={iconSectionMode === SectionMode.None || iconLayerMode === LayerMode.CurrentOnly}
                />
                <VisualSizeSelector 
                    label="Primary icon size"
                    hint="Size of primary icons. Which icons are primary depends on the route."
                    strings={GraphicSizeStrings}
                    value={primaryIconSize}
                    setValue={setPrimaryIconSize}
                />
                <VisualSizeSelector 
                    label="Secondary icon size"
                    hint="Size of secondary icons. Which icons are secondary depends on the route."
                    strings={GraphicSizeStrings}
                    value={secondaryIconSize}
                    setValue={setSecondaryIconSize}
                />
                <VisualSizeSelector 
                    label="Other icon size"
                    hint="Size of all other icons"
                    strings={GraphicSizeStrings}
                    value={otherIconSize}
                    setValue={setOtherIconSize}
                />
            </SettingsSection>
            <SettingsSection title="Lines">
                <SectionModeSelector value={lineSectionMode} setValue={setLineSectionMode} />
                <LayerModeSelector value={lineLayerMode} setValue={setLineLayerMode} />
                <Switch label="Fade lines not on current layer"
                    checked={!!fadeNonCurrentLayerLines}
                    onChange={(_, data) => setFadeNonCurrentLayerLines(data.checked)}
                    disabled={lineSectionMode === SectionMode.None || lineLayerMode === LayerMode.CurrentOnly}
                />
                <VisualSizeSelector 
                    label="Line thickness"
                    hint="Select how thick the lines should be"
                    strings={LineSizeStrings}
                    value={lineSize}
                    setValue={setLineSize}
                />
            </SettingsSection>
            <SettingsSection title="Markers">
                <SectionModeSelector value={markerSectionMode} setValue={setMarkerSectionMode} />
                <LayerModeSelector value={markerLayerMode} setValue={setMarkerLayerMode} />
                <Switch label="Fade markers not on current layer"
                    checked={!!fadeNonCurrentLayerMarkers}
                    onChange={(_, data) => setFadeNonCurrentLayerMarkers(data.checked)}
                    disabled={markerSectionMode === SectionMode.None || markerLayerMode === LayerMode.CurrentOnly}
                />
                <VisualSizeSelector 
                    label="Marker size"
                    hint="Select how big the circle markers should be"
                    strings={GraphicSizeStrings}
                    value={markerSize}
                    setValue={setMarkerSize}
                />
            </SettingsSection>
        </>
    );
};

/// Props for SectionModeSelector
type SectionModeSelectorProps = {
    /// Selected value
    value: SectionMode;
    /// Callback to set value
    setValue: (value: SectionMode) => void
}

/// Text displayed in the section mode selector dropdown
const SectionModeTexts = {
    [SectionMode.All]: "Show all sections",
    [SectionMode.Current]: "Show only current section",
    [SectionMode.None]: "Show nothing",
};

/// Section mode selector component
const SectionModeSelector: React.FC<SectionModeSelectorProps> = ({ value, setValue }) => {
    return (
        <Field
            label="Section mode"
            hint="Select how sections should be displayed on the map"
        >
            <Dropdown
                value={SectionModeTexts[value]}
                selectedOptions={[value]}
                onOptionSelect={(_, data) => {
                    setValue(data.selectedOptions[0] as SectionMode);
                }}
            >
                <Option text={SectionModeTexts[SectionMode.All]} value={SectionMode.All}>
                    All
                </Option>
                <Option text={SectionModeTexts[SectionMode.Current]} value={SectionMode.Current}>
                    Current section
                </Option>
                <Option text={SectionModeTexts[SectionMode.None]} value={SectionMode.None}>
                    None
                </Option>
            </Dropdown>
        </Field>
    )
}

/// Props for LayerModeSelector
type LayerModeSelectorProps = {
    /// Selected value
    value: LayerMode;
    /// Callback to set value
    setValue: (value: LayerMode) => void
}

/// Text displayed in the layer mode selector dropdown
const LayerModeTexts = {
    [LayerMode.CurrentOnly]: "Show only current layer",
    [LayerMode.CurrentAndAdjacent]: "Show current and adjacent layers",
    [LayerMode.All]: "Show all layers",
};

/// Layer mode selector component
const LayerModeSelector: React.FC<LayerModeSelectorProps> = ({ value, setValue }) => {
    return (
        <Field
            label="Layer mode"
            hint="Select how layers should be displayed on the map"
        >
            <Dropdown
                value={LayerModeTexts[value]}
                selectedOptions={[value]}
                onOptionSelect={(_, data) => {
                    setValue(data.selectedOptions[0] as LayerMode);
                }}
            >
                <Option text={LayerModeTexts[LayerMode.CurrentOnly]} value={LayerMode.CurrentOnly}>
                    Current layer only
                </Option>
                <Option text={LayerModeTexts[LayerMode.CurrentAndAdjacent]} value={LayerMode.CurrentAndAdjacent}>
                    Current and adjacent layers
                </Option>
                <Option text={LayerModeTexts[LayerMode.All]} value={LayerMode.All}>
                    All layers
                </Option>
            </Dropdown>
        </Field>
    )
}

/// Props for the VisualSizeSelector
type VisualSizeSelectorProps = {
    /// Label for the selector
    label: string
    /// Hint for the selector
    hint: string
    /// UI strings for the size value
    strings: Record<VisualSize, string>
    /// Selected value
    value: VisualSize,
    /// Callback to set value
    setValue: (value: VisualSize) => void
}

/// VisualSizeSelector component
const VisualSizeSelector: React.FC<VisualSizeSelectorProps> = ({ label, hint, strings, value, setValue }) => {
    return (
       <Field
            label={label}
            hint={hint}
       > 
            <div className="settings-value-slider">
                <Slider min={0} max={VisualSize.Large} step={1} value={value} onChange={(_, data)=>setValue(data.value as VisualSize)} />
                <Label>{strings[value]}</Label>
            </div>
       </Field>
    );
}

/// Strings for icon and marker sizes
const GraphicSizeStrings = {
    [VisualSize.Hidden]: "Hidden",
    [VisualSize.Small]: "Small",
    [VisualSize.Regular]: "Regular",
    [VisualSize.Large]: "Large",
}

/// Strings for line thickness
const LineSizeStrings = {
    [VisualSize.Hidden]: "None",
    [VisualSize.Small]: "Thin",
    [VisualSize.Regular]: "Regular",
    [VisualSize.Large]: "Thick",
}

