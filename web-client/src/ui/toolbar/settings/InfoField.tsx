//! Component for displaying one row of information with a label and a value

import { Label, Text } from "@fluentui/react-components";

type InfoFieldProps = {
    label: string;
    value: string;
};

export const InfoField: React.FC<InfoFieldProps> = ({ label, value }) => {
    return (
        <div className="settings-info-field">
            <Label>{label}</Label>
            <div>
                <Text>{value}</Text>
            </div>
        </div>
    );
};
