//! The error screen component

import { Body1, Button, Subtitle1, makeStyles, shorthands } from "@fluentui/react-components";

import { saveLog } from "low/utils";

const useStyles = makeStyles({
    container: {
        backgroundColor: "#555",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        height: "100%",
        color: "#ffa49d",
    },
    logo: {
        width: "100%",
        maxWidth: "128px",
    },
    message: {
        width: "100%",
        boxSizing: "border-box",
        position: "relative",
        marginTop: "20px",
        ...shorthands.padding("10px"),
    }
});

type ErrorScreenProps = {
    /// The error message to display
    message: string;
};

export const ErrorScreen: React.FC<ErrorScreenProps> = ({ message }) => {
    const styles = useStyles();
    return (
        <div className={styles.container}>
            <img className={styles.logo} src={"/static/celer-red.svg"} />
            <Subtitle1 block>Oops, something went wrong</Subtitle1>
            <div className={styles.message}>
                <Body1>{message}</Body1>
            </div>
            <Button appearance="primary" onClick={saveLog}>
                Download logs
            </Button>
        </div>
    );
};
