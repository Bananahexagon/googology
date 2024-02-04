import { parse } from "./parser";

const input = document.querySelector("#input") as HTMLTextAreaElement;
const button = document.querySelector("#button") as HTMLTextAreaElement;
const output = document.querySelector("#result") as HTMLTextAreaElement;
button.addEventListener("click", () => main(input.value));

function main(input: string) {
    const ast = parse(input);
    console.log(ast);
}