import {
    Folder16Filled, 
    Document16Filled,
    Info16Filled,
    CodeJs16Filled,
    CodeTs16Filled,
    CodePy16Filled,
    CodeBlock16Filled,
} from '@fluentui/react-icons';
import clsx from 'clsx';

export type TreeIconProps = {
    // If this is true, then the icon will be a folder icon.
    isDirectory: boolean;
    // File name used to determine the icon.
    file: string;
}

const getFileTypeAndIcon = ({isDirectory, file}: TreeIconProps): [string, JSX.Element] => {
    if (isDirectory) {
        return ["folder", <Folder16Filled />];
    }
    if (file.endsWith(".js") || file.endsWith(".jsx") || file.endsWith(".mjs") || file.endsWith(".cjs")) {
        return ["js", <CodeJs16Filled />];
    }
    if (file.endsWith(".ts") || file.endsWith(".tsx")) {
        return ["ts", <CodeTs16Filled />];
    }
    if (file.endsWith(".py")) {
        return ["py", <CodePy16Filled />];
    }
    if (file.endsWith(".json")) {
        return ["json", <CodeBlock16Filled />];
    }
    if (file.endsWith(".yaml") || file.endsWith(".yml")) {
        return ["yaml", <CodeBlock16Filled />];
    }
    if (file.endsWith(".md")) {
        return ["md", <Info16Filled />];
    }
    return ["unknown", <Document16Filled />];
}

export const TreeIcon: React.FC<TreeIconProps> = (props) => {
    const [fileType, icon] = getFileTypeAndIcon(props);
    return (
        <span className={clsx("editor-tree-item-icon", `file-type-${fileType}`)}>
            {icon}
        </span>
    );
}
