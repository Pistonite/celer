/// Create a yielder that can be used to yield to UI thread when the budget runs out
///
/// The returned function returns a promise that resolves to true
/// after yielding and regains the control. It resolves to false immediately if there
/// is still budget left
export const createYielder = (budget: number) => {
    let currentBudget = budget;
    return (cost: number = 1) => {
        if (currentBudget <= 0) {
            currentBudget = budget;
            return new Promise<boolean>((resolve) => {
                setTimeout(() => resolve(true), 0);
            });
        }
        currentBudget -= cost;
        return Promise.resolve(false);
    };
};
export type Yielder = ReturnType<typeof createYielder>;
