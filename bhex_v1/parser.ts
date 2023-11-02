import { AST, PT, zero, one, omega, mahlo } from "./define.js"

export function parse(input: string): AST {
    let result: AST | undefined = undefined;
    for (let i = 0; i < input.length; i++) {
        console.log(input[i]);
        switch (input[i]) {
            case "p": {
                let inner = "";
                i += 2;
                let stage = 1;
                loop: while (true) {
                    switch (input[i]) {
                        case "(": {
                            stage++;
                        } break;
                        case ")": {
                            stage--;
                            if (stage == 0) {
                                break loop;
                            }
                        } break;
                        case " ": { i++; continue; } break;
                        default: { } break;
                    }
                    inner += input[i];
                    i++;
                }
                if (result === undefined) {
                    result = {
                        type: "psi", arg: parse(inner)
                    };
                } else {
                    result = {
                        type: "add", left: result as PT, right: {
                            type: "psi", arg: parse(inner)
                        }
                    };
                }
            } break;
            case "m": {
                let inner = "";
                i += 2;
                let stage = 1;
                loop: while (true) {
                    switch (input[i]) {
                        case "(": {
                            stage++;
                        } break;
                        case ")": {
                            stage--;
                            if (stage == 0) {
                                break loop;
                            }
                        } break;
                        case " ": { i++; continue; } break;
                        default: { } break;
                    }
                    inner += input[i];
                    i++;
                }
                if (result === undefined) {
                    result = {
                        type: "mahlo", arg: parse(inner)
                    };
                } else {
                    result = {
                        type: "add", left: result as PT, right: {
                            type: "mahlo", arg: parse(inner)
                        }
                    };
                }
            } break;
            case "+": { } break;
            case "$": {
                i++;
                if (result === undefined) {
                    result = { "0": zero, "1": one, "w": omega, "M": mahlo }[input[i]]!;
                } else {
                    result = {
                        type: "add", left: result as PT, right: { "1": one, "w": omega, "M": mahlo }[input[i]]!
                    };
                }
            } break;
            case "[": {
                let inner = "";
                i++;
                let stage = 1;
                loop: while (true) {
                    switch (input[i]) {
                        case "[": {
                            stage++;
                        } break;
                        case "]": {
                            stage--;
                            if (stage == 0) {
                                break loop;
                            }
                        } break;
                        case " ": { i++; continue; } break;
                        default: { } break;
                    }
                    inner += input[i];
                    i++;
                }
                result = {
                    type: "nth", arg: result!, nth: parse(inner)
                };
                i++;
            } break;
            case " ": { } break;
            default: throw new Error("invalid token");
        }
    }
    return result!;
}