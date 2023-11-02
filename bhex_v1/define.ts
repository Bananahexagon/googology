export const zero: AST = {
    type: "zero",
}

export const one: AST = {
    type: "psi",
    arg: zero,
}

export const omega: AST = {
    type: "psi",
    arg: one,
}

export const mahlo: AST = {
    type: "mahlo",
    arg: zero,
}

export type PT = {

    type: "nth",
    arg: AST,
    nth: AST,
} | {
    type: "psi",
    arg: AST,
} | {
    type: "mahlo",
    arg: AST,

}

export type AST = PT | {
    type: "add",
    left: PT,
    right: AST,
} | {
    type: "zero",
}