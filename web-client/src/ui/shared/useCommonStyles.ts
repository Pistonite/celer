import { makeStyles } from "@fluentui/react-components";
import { prefersColorScheme } from "low/utils";

export const useCommonStyles = makeStyles({
    colorSuccess: {
        [prefersColorScheme("light")]: {
            color: "#44cc44",
            fill: "#44cc44",
        },
        [prefersColorScheme("dark")]: {
            color: "#88ff88",
            fill: "#88ff88",
        },
    },
    colorProgress: {
        [prefersColorScheme("light")]: {
            color: "#cc8844",
            fill: "#cc8844",
        },
        [prefersColorScheme("dark")]: {
            color: "#ffcc88",
            fill: "#ffcc88",
        },
    },
    colorError: {
        color: "#ff8888",
        fill: "#ff8888",
    },
    spinningInfinite: {
        animationTimingFunction: "linear",
        animationIterationCount: "infinite",
        animationDuration: "1s",
        animationName: {
            from: {
                transform: "rotate(0deg)",
            },
            to: {
                transform: "rotate(360deg)",
            },
        },
    },
});

export type CommonStyles = ReturnType<typeof useCommonStyles>;
