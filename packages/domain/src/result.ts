/** Explicit domain results. Inputs at real service boundaries still require schema validation. */
export type Result<A, E extends string> =
  | { readonly ok: true; readonly value: A }
  | { readonly ok: false; readonly error: E };
export const ok = <A>(value: A): Result<A, never> => ({ ok: true, value });
export const err = <E extends string>(error: E): Result<never, E> => ({ ok: false, error });
