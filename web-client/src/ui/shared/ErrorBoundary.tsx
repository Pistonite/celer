import type { PropsWithChildren } from "react";
import React from "react";

import { ErrorScreen } from "./ErrorScreen";

type ErrorState = {
    hasError: boolean;
};

/// Error boundary component
export class ErrorBoundary extends React.Component<
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
            return <ErrorScreen message={this.error} />;
        }
        return this.props.children;
    }
}
