//! Component for rendering a loading spinner
import {
    makeStyles,
    mergeClasses,
    shorthands,
} from "@fluentui/react-components";

const useStyles = makeStyles({
    container: {
        backgroundColor: "#555",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        height: "100%",
        rowGap: "20px",
    },
    logo: {
        width: "100%",
        maxWidth: "128px",
    },
    bar: {
        width: "100%",
        maxWidth: "256px",
        boxSizing: "border-box",
        height: "28px",
        position: "relative",
        backgroundColor: "#555",
        ...shorthands.padding("10px"),
    },
    barSpan: {
        display: "block",
        height: "100%",
        ...shorthands.borderRadius("25px"),
        position: "relative",
        ...shorthands.overflow("hidden"),
        // 3d effect
        boxShadow: "inset 0 -1px 1px rgba(0, 0, 0, 0.4)",

        // animated stripes
        "&:after": {
            content: "''",
            position: "absolute",
            top: 0,
            left: 0,
            bottom: 0,
            right: 0,
            zIndex: 1,
            backgroundSize: "50px 50px",
            ...shorthands.overflow("hidden"),
            animationDuration: "0.7s",
            animationIterationCount: "infinite",
            animationTimingFunction: "linear",
            animationName: {
                "0%": {
                    backgroundPositionX: 0,
                    backgroundPositionY: 0,
                },
                "100%": {
                    backgroundPositionX: "50px",
                    backgroundPositionY: "50px",
                },
            },
            backgroundImage: `
                linear-gradient(
                    -45deg,
                    rgba(0, 0, 0, 0.15) 25%,
                    transparent 25%,
                    transparent 50%,
                    rgba(0, 0, 0, 0.15) 50%,
                    rgba(0, 0, 0, 0.15) 75%,
                    transparent 75%,
                    transparent
                )`,
        },
    },
    // theme colors
    green: {
        backgroundColor: "#adfeb8",
    },
    blue: {
        backgroundColor: "#b0bafd",
    },
    red: {
        backgroundColor: "#ffa49d",
    },
    yellow: {
        backgroundColor: "#fee199",
    },
});

/// Props for the loading component
type LoadScreenProps = {
    /// Color
    color: "green" | "red" | "blue" | "yellow";
};

export const LoadScreen: React.FC<LoadScreenProps> = ({ color }) => {
    const styles = useStyles();
    return (
        <div className={styles.container}>
            <img className={styles.logo} src={`/static/celer-${color}.svg`} />
            <div className={styles.bar}>
                <span
                    className={mergeClasses(styles.barSpan, styles[color])}
                ></span>
            </div>
        </div>
    );
};
