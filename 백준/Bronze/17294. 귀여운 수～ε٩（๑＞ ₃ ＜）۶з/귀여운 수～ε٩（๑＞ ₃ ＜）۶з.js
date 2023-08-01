const k = [...`${require('fs').readFileSync(0)}`.trim()].map(Number);
const d = k[0] - k[1];
console.log(new Set(k.map((v, i) => v + d * i)).size === 1 ? '◝(⑅•ᴗ•⑅)◜..°♡ 뀌요미!!' : '흥칫뿡!! <(￣ ﹌ ￣)>');