
import init, { expand } from "./pkg/ebocf.js";

init().then(() => {
    const input = document.querySelector("#input");
    const button = document.querySelector("#button");
    const output = document.querySelector("#result");
    button.addEventListener("click", () => main(input.value));

    function main(input) {
        const result = expand(input);
        output.value = result;
    }
});