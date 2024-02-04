const zero: AST = {
    type: "zero",
}

const one: AST = {
    type: "psi",
    arg: zero,
}

const omega: AST = {
    type: "psi",
    arg: one,
}

type PT = {
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

type AST = PT | {
    type: "add",
    left: PT,
    right: AST,
} | {
    type: "zero",
}

export {
    AST, PT, zero, one, omega
}