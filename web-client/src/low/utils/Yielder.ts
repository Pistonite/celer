/// Create a yielder that can be used to yield to UI thread when the budget runs out
export const createYielder = (budget: number) => {
    let currentBudget = budget;
    return () => {
        if (currentBudget <= 0) {
            currentBudget = budget;
            return new Promise((resolve) => {
                setTimeout(resolve, 0);
            });
        }
        currentBudget--;
        return Promise.resolve();
    };
};
export type Yielder = ReturnType<typeof createYielder>;
