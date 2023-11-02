import { AST, zero, one, omega, mahlo } from "./define.js"
import { calc } from "./calc.js"
import { parse } from "./parser.js";

export function le(s: AST, t: AST): boolean {
    return s == t || lt(s, t);
}

export function lt(s: AST, t: AST): boolean {
    if (s.type == "nth") s = parse(calc(s.arg, s.nth));
    if (t.type == "nth") t = parse(calc(t.arg, t.nth));

    if (t == zero) return false;
    if (t != zero && s == zero) return true;
    if (t != zero && s.type == "add") {
        const [a, b] = [s.left, s.right];
        if (t.type == "add") {
            const [c, d] = [t.left, t.right];
            return lt(a, c) || (a == c && lt(b, d))
        } else if (t.type == "psi" || t.type == "mahlo") return lt(a, t);
    };
    if (t != zero && s.type == "psi") {
        const a = s.arg;
        if (t.type == "add") {
            const [b, c] = [t.left, t.right]
            return le(s, b);
        } else if (t.type == "psi") {
            const b = t.arg;
            return lt(a, b);
        } else if (t.type == "mahlo") {
            return true;
        }
    };
    if (t != zero && s.type == "mahlo") {
        const a = s.arg;
        if (t.type == "add") {
            const [b, c] = [t.left, t.right]
            return le(s, b);
        } else if (t.type == "psi") {
            return false;
        } else if (t.type == "mahlo") {
            const b = t.arg;
            return lt(a, b);
        }
    };
    return false;
}