import { saveLog } from "low/utils";
import type { PropsWithChildren } from "react";
import React from "react";

type ErrorState = {
    hasError: boolean;
};

/// Error boundary component
export class AppErrorBoundary extends React.Component<
    PropsWithChildren,
    ErrorState
> {
    state = { hasError: false };
    private error = "";

    static getDerivedStateFromError() {
        return { hasError: true };
    }

    componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
        this.error = `${error}\n${errorInfo.componentStack}`;
    }

    render() {
        if (this.state.hasError) {
            return (
                <>
                    <h1>Oh snap :(</h1>
                    <hr />
                    <p>
                        Celer has encountered an error and stopped working.
                        Refresh the page to reload it.
                    </p>
                    <p>Sorry for the inconvenience</p>
                    <p>{this.error}</p>
                    <button onClick={saveLog}>Download logs</button>
                </>
            );
        }
        return this.props.children;
    }
}
