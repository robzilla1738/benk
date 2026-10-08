import { err, ok, type Result } from "./result.js";
export interface Budget { readonly limit: string; readonly spent: string; readonly reserved: string }
export type BudgetError = "InvalidAmount" | "BudgetExceeded" | "ReservationMismatch";
const decimal = /^(0|[1-9][0-9]{0,18})$/;
function parse(value: string): bigint | undefined {
  return typeof value === "string" && decimal.test(value) ? BigInt(value) : undefined;
}
function fields(b: Budget): readonly [bigint, bigint, bigint] | undefined {
  const l=parse(b.limit), s=parse(b.spent), r=parse(b.reserved);
  return l === undefined || s === undefined || r === undefined ? undefined : [l,s,r];
}
export function reserveBudget(b: Budget, amount: string): Result<Budget, BudgetError> {
  const f=fields(b), a=parse(amount);
  if (!f || a === undefined) return err("InvalidAmount");
  const [limit, spent, reserved]=f;
  if (spent + reserved + a > limit) return err("BudgetExceeded");
  return ok({ limit:b.limit, spent:b.spent, reserved:(reserved+a).toString() });
}
/** Caller must settle a uniquely identified reservation once in a real transaction. */
export function settleBudget(b: Budget, reservation: string, actual: string): Result<{
  budget: Budget; overLimit: boolean; exceededReservation: boolean
}, BudgetError> {
  const f=fields(b), r=parse(reservation), a=parse(actual);
  if (!f || r === undefined || a === undefined) return err("InvalidAmount");
  const [limit, spent, reserved]=f;
  if (r > reserved) return err("ReservationMismatch");
  const nextSpent=spent+a, nextReserved=reserved-r;
  if (nextSpent.toString().length > 19) return err("InvalidAmount");
  return ok({ budget:{limit:b.limit,spent:nextSpent.toString(),reserved:nextReserved.toString()},
    overLimit:nextSpent+nextReserved>limit, exceededReservation:a>r });
}
