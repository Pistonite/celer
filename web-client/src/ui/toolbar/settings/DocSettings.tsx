//! Doc tab of the settings dialog

import {
    Body1,
    Button,
    Dropdown,
    Field,
    Switch,
    Option,
    Checkbox,
} from "@fluentui/react-components";
import { useSelector } from "react-redux";
import { useMemo } from "react";

import {
    documentSelector,
    settingsActions,
    settingsSelector,
    viewActions,
    viewSelector,
} from "core/store";
import {
    KeyBinding,
    KeyBindingName,
    getAllSplitTypes,
    useDocSplitTypes,
} from "core/doc";
import { useActions } from "low/store";
import { ThemeIds } from "low/themes.g";

import { SettingsSection } from "./SettingsSection";

export const DocSettings: React.FC = () => {
    const { serial, document } = useSelector(documentSelector);
    const { syncMapToDoc, forceAnchorNotes, hideDocWhenResizing } =
        useSelector(settingsSelector);
    const {
        setSyncMapToDoc,
        setForceAnchorNotes,
        setHideDocWhenResizing,
        setSplitTypes,
    } = useActions(settingsActions);

    /* eslint-disable react-hooks/exhaustive-deps*/
    const allSplitTypes = useMemo(() => {
        if (document) {
            return getAllSplitTypes(document);
        }
        return [];
    }, [serial]);
    /* eslint-enable react-hooks/exhaustive-deps*/

    const currentSplitTypes = useDocSplitTypes();

    return (
        <>
            <SettingsSection title="Appearance">
                <ThemeSelector />
                <Field
                    label="Hide document when editing layout"
                    hint="Automatically hide the document when the layout is being edited, which reduces lag when moving things around."
                >
                    <Switch
                        checked={!!hideDocWhenResizing}
                        onChange={(_, data) =>
                            setHideDocWhenResizing(data.checked)
                        }
                    />
                </Field>
                <Field
                    label="Always anchor notes to corresponding line"
                    hint="When disabled, notes are allowed to shift around to avoid overlapping. The note of the current line will always be brought to top when navigating the document."
                >
                    <Switch
                        checked={!!forceAnchorNotes}
                        onChange={(_, data) =>
                            setForceAnchorNotes(data.checked)
                        }
                    />
                </Field>
            </SettingsSection>
            <SettingsSection title="Keyboard control">
                <Body1 block>
                    To changing a key binding, click on the button, then press
                    and hold the key(s) you want to use.
                </Body1>
                <Body1 block>
                    Note that some keys may conflict with the default browser
                    behavior and/or the map behavior when focused on the map
                </Body1>
                <KeyBindingEditor
                    name="prevLineKey"
                    label="Previous line"
                    hint="Move up one line in the document"
                />
                <KeyBindingEditor
                    name="nextLineKey"
                    label="Next line"
                    hint="Move down one line in the document"
                />
                <KeyBindingEditor
                    name="prevSplitKey"
                    label="Previous split"
                    hint="Move to the previous split (depends on split settings)"
                />
                <KeyBindingEditor
                    name="nextSplitKey"
                    label="Next split"
                    hint="Move to the next split (depends on split settings)"
                />
            </SettingsSection>
            <SettingsSection title="Splits">
                <Field
                    label="Choose where you want to split"
                    validationState={
                        allSplitTypes.length === 0 ? "warning" : undefined
                    }
                    validationMessage={
                        allSplitTypes.length === 0
                            ? "The document doesn't define any split type."
                            : undefined
                    }
                    hint={
                        allSplitTypes.length === 0
                            ? undefined
                            : "Check the boxes below to enable splitting on the corresponding types of items. Click the button to reset to the document default."
                    }
                >
                    <Button
                        appearance="primary"
                        onClick={() => setSplitTypes(undefined)}
                    >
                        Reset split types
                    </Button>
                </Field>
                {allSplitTypes.map((type, i) => {
                    return (
                        <Checkbox
                            key={i}
                            label={type}
                            checked={currentSplitTypes.includes(type)}
                            onChange={(_, data) => {
                                if (data.checked) {
                                    setSplitTypes([...currentSplitTypes, type]);
                                } else {
                                    setSplitTypes(
                                        currentSplitTypes.filter(
                                            (x) => x !== type,
                                        ),
                                    );
                                }
                            }}
                        />
                    );
                })}
            </SettingsSection>
            <SettingsSection title="Map integration">
                <Field
                    label="Sync map view"
                    hint="Automatically fit the map view when scrolling through the document so that all items currently visible in the document are also visible on the map"
                >
                    <Switch
                        checked={!!syncMapToDoc}
                        onChange={(_, data) => setSyncMapToDoc(data.checked)}
                    />
                </Field>
            </SettingsSection>
        </>
    );
};

/// Input control for editing key binding
type KeyBindingEditorProps = {
    /// Name of the key binding
    name: KeyBindingName;
    /// Display label
    label: string;
    /// Hint to display
    hint: string;
};
const KeyBindingEditor: React.FC<KeyBindingEditorProps> = ({
    name,
    label,
    hint,
}) => {
    const { editingKeyBinding } = useSelector(viewSelector);
    const keyBinding = useSelector(settingsSelector)[name];
    const { setEditingKeyBinding } = useActions(viewActions);
    return (
        <Field label={label} hint={hint}>
            <Button onClick={() => setEditingKeyBinding(name)}>
                {editingKeyBinding === name
                    ? "Waiting for key..."
                    : keyToString(keyBinding)}
            </Button>
        </Field>
    );
};

/// Helper to display a key binding
const keyToString = (key: KeyBinding): string => {
    return key.map((x) => (x.length === 1 ? x.toUpperCase() : x)).join(" + ");
};

/// Theme selector control
const ThemeSelector: React.FC = () => {
    const { theme } = useSelector(settingsSelector);
    const { setDocTheme } = useActions(settingsActions);
    return (
        <Field label="Theme" hint="Change how the document viewer looks">
            <Dropdown
                value={themeIdToDisplayName(theme)}
                selectedOptions={[theme]}
                onOptionSelect={(_, data) => {
                    setDocTheme(data.selectedOptions[0]);
                }}
            >
                {ThemeIds.map((id) => {
                    const text = themeIdToDisplayName(id);
                    return (
                        <Option key={id} text={text} value={id}>
                            {text}
                        </Option>
                    );
                })}
            </Dropdown>
        </Field>
    );
};
/// Convert snake_case or kebab-case to Pascal Case
const themeIdToDisplayName = (id: string) => {
    if (!id) {
        return "Unknown";
    }
    return (
        id[0].toUpperCase() +
        id
            .slice(1)
            .replace(/[-_](\w)/g, (_, letter) => " " + letter.toUpperCase())
            .trim()
    );
};
