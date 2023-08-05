const s = 'ㄱㄲㄴㄷㄸㄹㅁㅂㅃㅅㅆㅇㅈㅉㅊㅋㅌㅍㅎ';
const f = w => s[Math.floor((w.charCodeAt(0) - 0xac00 - ((w.charCodeAt(0) - 0xac00) % 28)) / 28 / 21)];
console.log([...`${require('fs').readFileSync(0)}`.trim()].map(f).join(''));