import init, { expand } from "./pkg/bhex_v1.js";

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


const input = document.querySelector("#input");
input.value =
    `0
p(0)
1+1
2+1
3+1
p(1)
w[4]
w+w[4]
p(2)[4]
p(w)[4]
p(w+1)[4]
p(w+w)[4]
p(p(2))[4]
m(0)
p(M)
p(W) [4]
p(W+1) [4]
p(W+w) [4]
p(W+p(W)) [4]
p(W+W) [4]
p(W+W+W) [4]
p(p(M+1))[4]
p(p(M+1)+W)[4]
p(p(M+1)+p(M+1))[4]
p(p(M+2))[4]
p(p(M+w))[4]
p(p(M+W))[4]
p(p(M+W+W))[4]
p(p(M+p(M+1)))[4]
p(p(M+M))[4]
p(p(M+M+W))[4]
p(p(M+M+M))[4]
p(p(m(1)))[4]
p(p(m(w)))[4]
p(p(m(W)))[4]
p(p(m(M)))[4]
p(p(m(m(M))))[4]`