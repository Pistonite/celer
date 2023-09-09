//! Control for selecting section in document
//!
//! Clicking on a section will scoll the document to the first line of the section
//! (For empty sections, scroll to the section header)

import {
    Menu,
    MenuItem,
    MenuItemRadio,
    MenuList,
    MenuPopover,
    MenuTrigger,
    ToolbarButton,
    Tooltip,
} from "@fluentui/react-components";
import { ListBarTree20Regular } from "@fluentui/react-icons";
import isEqual from "is-equal";
import React, { forwardRef } from "react";
import { useSelector } from "react-redux";
import { useDocSections } from "core/doc";
import { viewActions, viewSelector } from "core/store";
import { useActions } from "low/store";

import {
    ControlComponentProps,
    OnMenuCheckedValueChangeFunction,
    ToolbarControl,
} from "./util";

export const SelectSection: ToolbarControl = {
    ToolbarButton: forwardRef<HTMLButtonElement>((_, ref) => {
        const sections = useDocSections();
        const { currentSection } = useSelector(viewSelector);
        const { setDocLocation } = useActions(viewActions);
        return (
            <SelectSectionInternal
                sections={sections}
                current={currentSection}
                selectSection={(section) => {
                    setDocLocation({ section, line: 0 });
                }}
            >
                <Tooltip content="Jump to section" relationship="label">
                    <ToolbarButton
                        disabled={sections.length === 0}
                        icon={<ListBarTree20Regular />}
                        ref={ref}
                    />
                </Tooltip>
            </SelectSectionInternal>
        );
    }),
    MenuItem: () => {
        const sections = useDocSections();
        const { currentSection } = useSelector(viewSelector);
        const { setDocLocation } = useActions(viewActions);
        return (
            <SelectSectionInternal
                sections={sections}
                current={currentSection}
                selectSection={(section) => {
                    setDocLocation({ section, line: 0 });
                }}
            >
                <MenuItem
                    icon={<ListBarTree20Regular />}
                    disabled={sections.length === 0}
                >
                    Jump to section
                </MenuItem>
            </SelectSectionInternal>
        );
    },
};

const SectionRadioName = "Select a section";
/// Internal component implementation
type SelectSectionInternalProps = ControlComponentProps & {
    /// Section names
    sections: string[];
    /// Current section
    current: number;
    /// Callback to select a section
    selectSection: (section: number) => void;
};
const SelectSectionInternal = React.memo(
    ({
        sections,
        current,
        selectSection,
        children,
    }: SelectSectionInternalProps) => {
        const checkedItems = {
            [SectionRadioName]: [`${current}`],
        };
        const onChangeCheckedItems: OnMenuCheckedValueChangeFunction = (
            _,
            { checkedItems },
        ) => {
            const section = parseInt(checkedItems[0]);
            selectSection(section);
        };
        return (
            <Menu
                checkedValues={checkedItems}
                onCheckedValueChange={onChangeCheckedItems}
            >
                <MenuTrigger>{children}</MenuTrigger>
                <MenuPopover>
                    <MenuList>
                        {sections.map((section, i) => (
                            <Tooltip
                                key={i}
                                content="Jump to this section"
                                relationship="description"
                            >
                                <MenuItemRadio
                                    name={SectionRadioName}
                                    value={`${i}`}
                                >
                                    {section}
                                </MenuItemRadio>
                            </Tooltip>
                        ))}
                    </MenuList>
                </MenuPopover>
            </Menu>
        );
    },
    (prev, next) => {
        return (
            isEqual(prev.sections, next.sections) &&
            prev.current === next.current
        );
    },
);
