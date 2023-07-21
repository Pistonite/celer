import { useStore } from "react-redux";

import { AppStore } from "core/store";

/// Convenience hook for typing the redux store correctly
export const useAppStore: () => AppStore = useStore as () => AppStore;
