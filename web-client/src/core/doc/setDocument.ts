import { deepEqual } from "fast-equals";

import type { AppStore } from "core/store";
import {
    documentActions,
    settingsActions,
    settingsSelector,
    viewActions,
} from "core/store";

import type { ExpoContext } from "low/celerc";
import { consoleDoc as console } from "low/utils";

export function setDocument(store: AppStore, doc: ExpoContext | undefined) {
    if (!doc) {
        store.dispatch(documentActions.setDocument(undefined));
        return;
    }

    const title = doc.execDoc.project.title;
    const { pluginMetadatas } = settingsSelector(store.getState());
    if (deepEqual(doc.pluginMetadata, pluginMetadatas[title])) {
        store.dispatch(documentActions.setDocument(doc));
        return;
    }

    console.info("updating document plugin metadata");

    store.dispatch(viewActions.setSuppressRecompile(true));
    store.dispatch(
        settingsActions.setPluginMetadata({
            title,
            metadata: doc.pluginMetadata,
        }),
    );
    store.dispatch(documentActions.setDocument(doc));
    store.dispatch(viewActions.setSuppressRecompile(false));

    console.info("document plugin metadata updated");
}
