import { parse } from "./parser.js";
var input = document.querySelector("#input");
var button = document.querySelector("#button");
var output = document.querySelector("#result");
button.addEventListener("click", function () { return main(input.value); });
function main(input) {
    var ast = parse(input);
    console.log(ast);
}
