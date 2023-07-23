// jest env setup
import "@testing-library/jest-dom";

/* eslint @typescript-eslint/no-explicit-any: 0 */

/// Create a mock store for testing
export const createMockStore = (initialState: unknown): any => {
    return {
        getState: () => initialState,
        subscribe: jest.fn(),
        dispatch: jest.fn(),
    } as any;
};

/// Access a member of an object, asserting that it exists
///
/// Use this to access private members for testing
export const getAttr = (obj: unknown, member: string) => {
    expect(obj).toHaveProperty(member);
    return (obj as any)[member];
};
