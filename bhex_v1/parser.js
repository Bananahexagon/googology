export function parse(input) {
    var result = undefined;
    for (var i = 0; i < input.length; i++) {
        console.log(input[i]);
        switch (input[i]) {
            case "p":
                {
                    var inner = "";
                    i += 2;
                    var stage = 1;
                    loop: while (true) {
                        switch (input[i]) {
                            case "(":
                                {
                                    stage++;
                                }
                                break;
                            case ")":
                                {
                                    stage--;
                                    if (stage == 0) {
                                        break loop;
                                    }
                                }
                                break;
                            case " ":
                                {
                                    i++;
                                    continue;
                                }
                                break;
                            default:
                                { }
                                break;
                        }
                        inner += input[i];
                        i++;
                    }
                    if (result === undefined) {
                        result = {
                            type: "psi", arg: parse(inner)
                        };
                    }
                    else {
                        result = {
                            type: "add", left: result, right: {
                                type: "psi", arg: parse(inner)
                            }
                        };
                    }
                }
                break;
            case "m":
                {
                    var inner = "";
                    i += 2;
                    var stage = 1;
                    loop: while (true) {
                        switch (input[i]) {
                            case "(":
                                {
                                    stage++;
                                }
                                break;
                            case ")":
                                {
                                    stage--;
                                    if (stage == 0) {
                                        break loop;
                                    }
                                }
                                break;
                            case " ":
                                {
                                    i++;
                                    continue;
                                }
                                break;
                            default:
                                { }
                                break;
                        }
                        inner += input[i];
                        i++;
                    }
                    if (result === undefined) {
                        result = {
                            type: "mahlo", arg: parse(inner)
                        };
                    }
                    else {
                        result = {
                            type: "add", left: result, right: {
                                type: "mahlo", arg: parse(inner)
                            }
                        };
                    }
                }
                break;
            case "+":
                { }
                break;
            case "$":
                {
                    i++;
                    if (result === undefined) {
                        result = { "0": zero, "1": one, "w": omega, "M": mahlo }[input[i]];
                    }
                    else {
                        result = {
                            type: "add", left: result, right: { "1": one, "w": omega, "M": mahlo }[input[i]],
                        };
                    }
                }
                break;
            case "[":
                {
                    var inner = "";
                    i++;
                    var stage = 1;
                    loop: while (true) {
                        switch (input[i]) {
                            case "[":
                                {
                                    stage++;
                                }
                                break;
                            case "]":
                                {
                                    stage--;
                                    if (stage == 0) {
                                        break loop;
                                    }
                                }
                                break;
                            case " ":
                                {
                                    i++;
                                    continue;
                                }
                                break;
                            default:
                                { }
                                break;
                        }
                        inner += input[i];
                        i++;
                    }
                    result = {
                        type: "nth", arg: result, nth: parse(inner)
                    };
                    i++;
                }
                break;
            case " ":
                { }
                break;
            default: throw new Error("invalid token");
        }
    }
    return result;
}
var zero = {
    type: "zero",
};
var one = {
    type: "psi",
    arg: zero,
};
var omega = {
    type: "psi",
    arg: one,
};
var mahlo = {
    type: "mahlo",
    arg: zero,
};
