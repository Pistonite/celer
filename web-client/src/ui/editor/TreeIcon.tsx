import { mergeClasses } from "@fluentui/react-components";
import {
    Folder16Filled,
    Document16Regular,
    Image16Filled,
    Info16Filled,
    CodeJs16Filled,
    CodeTs16Filled,
    CodePy16Filled,
    CodeBlock16Filled,
} from "@fluentui/react-icons";

import { useEditorStyles } from "./styles";

export type TreeIconProps = {
    // If this is true, then the icon will be a folder icon.
    isDirectory: boolean;
    // File name used to determine the icon.
    file: string;
};

const getFileTypeAndIcon = ({ isDirectory, file }: TreeIconProps) => {
    if (isDirectory) {
        return ["Folder", <Folder16Filled />] as const;
    }
    if (file.match(/\.(m|c)?jsx?$/i)) {
        return ["Js", <CodeJs16Filled />] as const;
    }
    if (file.match(/\.(m|c)?tsx?$/i)) {
        return ["Ts", <CodeTs16Filled />] as const;
    }
    if (file.match(/\.py$/i)) {
        return ["Py", <CodePy16Filled />] as const;
    }
    if (file.match(/\.json$/i)) {
        return ["Json", <CodeBlock16Filled />] as const;
    }
    if (file.match(/\.ya?ml$/i)) {
        return ["Yaml", <CodeBlock16Filled />] as const;
    }
    if (file.match(/\.md$/i)) {
        return ["Md", <Info16Filled />] as const;
    }
    if (file.match(/\.(png|jpe?g|gif|webp)$/i)) {
        return ["Image", <Image16Filled />] as const;
    }
    return ["Unknown", <Document16Regular />] as const;
};

export const TreeIcon: React.FC<TreeIconProps> = (props) => {
    const styles = useEditorStyles();
    const [fileType, icon] = getFileTypeAndIcon(props);
    return (
        <span
            className={mergeClasses(
                styles.editorTreeItemIcon,
                styles[`fileType${fileType}`],
            )}
        >
            {icon}
        </span>
    );
};
