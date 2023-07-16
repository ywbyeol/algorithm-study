const s = `${require('fs').readFileSync(0)}`.trim();
const l = s.length / 3;
console.log(s.slice(0, l) === s.slice(l, l * 2) ? s.slice(0, l) : s.slice(l * 2));