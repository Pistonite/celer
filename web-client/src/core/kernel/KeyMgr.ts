//! Logic for handling key events and bindings

import type { AppStore } from "core/store";
import {
    documentSelector,
    settingsActions,
    settingsSelector,
    viewActions,
    viewSelector,
} from "core/store";
import {
    getDefaultSplitTypes,
    getRelativeLocation,
    getRelativeSplitLocation,
} from "core/doc";

/// Manager for key events and bindings
///
/// Connects to the store for handling the key
export class KeyMgr {
    /// The store to operate on
    private store: AppStore;
    /// The current keys that are held down
    private currentStrokes: string[] = [];
    /// The current detected key binding
    ///
    /// The key manager behaves differently when detecting a key binding
    /// vs. a key binding is already detected, and currently is waiting
    /// for it to be released.
    private lastDetected: string[] = [];

    private cleanupFn: (() => void) | undefined = undefined;

    constructor(store: AppStore) {
        this.store = store;
    }

    /// Add listeners to the window
    public init() {
        const onKeyDown = (e: KeyboardEvent) => {
            this.onKeyDown(e.key);
        };
        const onKeyUp = (e: KeyboardEvent) => {
            this.onKeyUp(e.key);
        };
        window.addEventListener("keydown", onKeyDown);
        window.addEventListener("keyup", onKeyUp);
        this.cleanupFn = () => {
            window.removeEventListener("keydown", onKeyDown);
            window.removeEventListener("keyup", onKeyUp);
        };
    }

    /// Remove all listeners
    public delete() {
        if (this.cleanupFn) {
            this.cleanupFn();
        }
    }

    /// Handle when a key is pressed
    ///
    /// This will add to the current pressed strokes.
    /// If not currently editing a key binding, and the current strokes
    /// match an action, it will execute that action
    public onKeyDown(key: string) {
        if (this.isEditingKeyBinding()) {
            this.handleAddKey(key);
        } else if (this.lastDetected.length === 0) {
            // detecting mode
            this.handleAddKey(key);
            const { prevLineKey, nextLineKey, prevSplitKey, nextSplitKey } =
                settingsSelector(this.store.getState());
            if (this.keySequenceMatches(prevLineKey)) {
                this.handleMoveLineAction(-1);
                this.lastDetected = prevLineKey;
            } else if (this.keySequenceMatches(nextLineKey)) {
                this.handleMoveLineAction(1);
                this.lastDetected = nextLineKey;
            } else if (this.keySequenceMatches(prevSplitKey)) {
                this.handleMoveSplitAction(-1);
                this.lastDetected = prevSplitKey;
            } else if (this.keySequenceMatches(nextSplitKey)) {
                this.handleMoveSplitAction(1);
                this.lastDetected = nextSplitKey;
            }
        } else {
            // waiting for release
            this.handleAddKey(key);
        }
    }

    /// Add key to current stroke if it's not already in it
    private handleAddKey(key: string) {
        if (this.currentStrokes.includes(key)) {
            return;
        }
        this.currentStrokes.push(key);
    }

    /// Handle when a key is released
    ///
    /// If editing a key binding, this will update the key binding.
    /// Otherwise it will transition the state
    public onKeyUp(key: string) {
        if (this.isEditingKeyBinding()) {
            this.updateKeyBinding();
        }
        // remove the release key from the current strokes
        const i = this.currentStrokes.indexOf(key);
        if (i !== -1) {
            this.currentStrokes.splice(i, 1);
        }

        if (this.lastDetected.length > 0) {
            // check if the key binding was released
            if (!this.keySequenceMatches(this.lastDetected)) {
                // release detected
                this.lastDetected = [];
            }
        }
    }

    private isEditingKeyBinding() {
        return (
            viewSelector(this.store.getState()).editingKeyBinding !== undefined
        );
    }

    /// Check if the current key sequence matches the expected sequence
    ///
    /// Matches if the last (expected.length) keys match
    private keySequenceMatches(expected: string[]) {
        if (this.currentStrokes.length < expected.length) {
            return false;
        }
        for (let i = 0; i < expected.length; i++) {
            // i is element from end
            const iCurrent = this.currentStrokes.length - 1 - i;
            const iExpected = expected.length - 1 - i;
            if (this.currentStrokes[iCurrent] !== expected[iExpected]) {
                return false;
            }
        }
        return true;
    }

    /// Handle key binding action for moving documents by line
    private handleMoveLineAction(delta: number) {
        const { document } = documentSelector(this.store.getState());
        if (!document) {
            return;
        }
        const { currentSection, currentLine } = viewSelector(
            this.store.getState(),
        );
        const nextLocation = getRelativeLocation(
            document,
            currentSection,
            currentLine,
            delta,
        );
        this.store.dispatch(viewActions.setDocLocation(nextLocation));
    }

    /// Handle key binding action for moving documents by split
    private handleMoveSplitAction(delta: number) {
        const { document } = documentSelector(this.store.getState());
        if (!document) {
            return;
        }
        let { splitTypes } = settingsSelector(this.store.getState());
        if (!splitTypes) {
            splitTypes = getDefaultSplitTypes(document);
        }
        const { currentSection, currentLine } = viewSelector(
            this.store.getState(),
        );
        const nextLocation = getRelativeSplitLocation(
            document,
            currentSection,
            currentLine,
            delta,
            splitTypes,
        );
        this.store.dispatch(viewActions.setDocLocation(nextLocation));
    }

    /// Update the current editing keybinding in the store
    private updateKeyBinding() {
        if (this.currentStrokes.length === 0) {
            // safety check since we want to avoid having empty key bindings
            return;
        }
        const editingKeyBinding = viewSelector(
            this.store.getState(),
        ).editingKeyBinding;
        if (editingKeyBinding === undefined) {
            return;
        }

        // stop editing key binding
        this.store.dispatch(viewActions.setEditingKeyBinding(undefined));
        // update the binding
        this.store.dispatch(
            settingsActions.setDocKeyBinding({
                name: editingKeyBinding,
                // create a copy to avoid reference bugs
                value: [...this.currentStrokes],
            }),
        );
    }
}
