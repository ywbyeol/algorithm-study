const i = `${require('fs').readFileSync(0)}`.trim();
const a = i.slice(0, Math.floor(i.length / 2));
const b = [...i.slice(Math.ceil(i.length / 2))].reverse().join('');
console.log(a === b ? 1 : 0);