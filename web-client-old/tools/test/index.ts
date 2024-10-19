//! testing utilities
//!
//! Things here should be used for testing only!!

/* eslint @typescript-eslint/no-explicit-any: 0 */

/// Create a mock store for testing
export const createMockStore = (initialState: any): any => {
    return {
        getState: () => initialState,
        subscribe: jest.fn(),
        dispatch: jest.fn(),
    } as any;
};

/// Access a member of an object, asserting that it exists
///
/// Use this to access private members for testing
export const getAttr = (obj: any, member: string) => {
    expect(obj).toHaveProperty(member);
    return (obj as any)[member];
};

/// Mutate a member of an object, asserting that it exists
///
/// Use this to access private members for testing
export const setAttr = (obj: any, member: string, value: any) => {
    expect(obj).toHaveProperty(member);
    (obj as any)[member] = value;
};
