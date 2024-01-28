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


let digitosNF = obterDigitosNF();

export function formatarValor(v: number) {
    let s: string = `${v}`;
    while (s.length < 3) s = '0' + s;
    let inteira = s.slice(0, s.length - 2);
    let pontuada =
        inteira.slice(0, inteira.length % 3) +
        inteira.slice(inteira.length % 3).replace(/.{3}/g, '.$&');
    if (pontuada[0] == '.') {
        pontuada = pontuada.slice(1);
    }
    //.split('').reverse().join('').replace(/.{3}/g, '$&.').split('').reverse().join('')
    return pontuada + ',' + s.slice(s.length - 2);
    //	562 322 2,48
}
export function formatarNF(nf: number) {
    let s: string = `${nf}`;
    while (s.length < digitosNF) s = '0' + s;
    return s;
}
export function formatarData(dataStr:string){
    let s = dataStr.split('-');
    return s[2]+'/'+s[1]+'/'+s[0];
}