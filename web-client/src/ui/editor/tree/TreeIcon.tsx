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
import clsx from "clsx";

export type TreeIconProps = {
    // If this is true, then the icon will be a folder icon.
    isDirectory: boolean;
    // File name used to determine the icon.
    file: string;
};

const getFileTypeAndIcon = ({
    isDirectory,
    file,
}: TreeIconProps): [string, JSX.Element] => {
    if (isDirectory) {
        return ["folder", <Folder16Filled />];
    }
    if ( file.match(/\.(m|c)?jsx?$/i)) {
        return ["js", <CodeJs16Filled />];
    }
    if ( file.match(/\.(m|c)?tsx?$/i)) {
        return ["ts", <CodeTs16Filled />];
    }
    if (file.match(/\.py$/i)) {
        return ["py", <CodePy16Filled />];
    }
    if (file.match(/\.json$/i)) {
        return ["json", <CodeBlock16Filled />];
    }
    if (file.match(/\.ya?ml$/i)) {
        return ["yaml", <CodeBlock16Filled />];
    }
    if (file.match(/\.md$/i)) {
        return ["md", <Info16Filled />];
    }
    if (file.match(/\.(png|jpe?g|gif|webp)$/i)) {
        return ["image", <Image16Filled />];
    }
    return ["unknown", <Document16Regular />];
};

export const TreeIcon: React.FC<TreeIconProps> = (props) => {
    const [fileType, icon] = getFileTypeAndIcon(props);
    return (
        <span
            className={clsx("editor-tree-item-icon", `file-type-${fileType}`)}
        >
            {icon}
        </span>
    );
};
