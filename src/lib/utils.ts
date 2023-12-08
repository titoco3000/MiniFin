export function obterDigitosNF() {
    let digitosNFstr = localStorage.getItem("digitosNF");
    let digitosNF = 9;
    if (digitosNFstr == null)
        localStorage.setItem("digitosNF", `{digitosNF}`);
    else {
        digitosNF = parseInt(digitosNFstr);
        if (isNaN(digitosNF)) {
            digitosNF = 9;
            localStorage.setItem("digitosNF", '9');
        }
    }
    return digitosNF;
}