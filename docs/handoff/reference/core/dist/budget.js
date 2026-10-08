import { err, ok } from "./result.js";
const decimal = /^(0|[1-9][0-9]{0,18})$/;
function parse(value) {
    return typeof value === "string" && decimal.test(value) ? BigInt(value) : undefined;
}
function fields(b) {
    const l = parse(b.limit), s = parse(b.spent), r = parse(b.reserved);
    return l === undefined || s === undefined || r === undefined ? undefined : [l, s, r];
}
export function reserveBudget(b, amount) {
    const f = fields(b), a = parse(amount);
    if (!f || a === undefined)
        return err("InvalidAmount");
    const [limit, spent, reserved] = f;
    if (spent + reserved + a > limit)
        return err("BudgetExceeded");
    return ok({ limit: b.limit, spent: b.spent, reserved: (reserved + a).toString() });
}
/** Caller must settle a uniquely identified reservation once in a real transaction. */
export function settleBudget(b, reservation, actual) {
    const f = fields(b), r = parse(reservation), a = parse(actual);
    if (!f || r === undefined || a === undefined)
        return err("InvalidAmount");
    const [limit, spent, reserved] = f;
    if (r > reserved)
        return err("ReservationMismatch");
    const nextSpent = spent + a, nextReserved = reserved - r;
    if (nextSpent.toString().length > 19)
        return err("InvalidAmount");
    return ok({ budget: { limit: b.limit, spent: nextSpent.toString(), reserved: nextReserved.toString() },
        overLimit: nextSpent + nextReserved > limit, exceededReservation: a > r });
}
//# sourceMappingURL=budget.js.map