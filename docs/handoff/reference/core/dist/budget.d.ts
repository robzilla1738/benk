import { type Result } from "./result.js";
export interface Budget {
    readonly limit: string;
    readonly spent: string;
    readonly reserved: string;
}
export type BudgetError = "InvalidAmount" | "BudgetExceeded" | "ReservationMismatch";
export declare function reserveBudget(b: Budget, amount: string): Result<Budget, BudgetError>;
/** Caller must settle a uniquely identified reservation once in a real transaction. */
export declare function settleBudget(b: Budget, reservation: string, actual: string): Result<{
    budget: Budget;
    overLimit: boolean;
    exceededReservation: boolean;
}, BudgetError>;
