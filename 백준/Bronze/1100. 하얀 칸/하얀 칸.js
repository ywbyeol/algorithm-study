let c = 0;
const b = `${require('fs').readFileSync(0)}`.trim().split(/\s+/).map((v, i) => (i % 2 !== 0 ? `.${v}` : v));
b.forEach(l => [...l].forEach((v, i) => (i % 2 === 0 && v === 'F' ? c++ : c)));
console.log(c);