import { Caption1, makeStyles } from "@fluentui/react-components";
import { PropsWithChildren } from "react";

const useStyles = makeStyles({
    container: {
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        width: "100%",
        height: "100%",
        color: "#888",
        textAlign: "center",
        "& span": {
            textAlign: "center",
        },
    },
});

export const HintScreen: React.FC<PropsWithChildren> = ({ children }) => {
    const styles = useStyles();
    return (
        <div className={styles.container}>
            <Caption1>{children}</Caption1>
        </div>
    );
};
