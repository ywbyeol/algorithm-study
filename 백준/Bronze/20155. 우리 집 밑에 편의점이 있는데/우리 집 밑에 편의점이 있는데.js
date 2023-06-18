const arr = `${require('fs').readFileSync(0)}`.trim().split(/\s+/).slice(2);
const map = new Map([...new Set(arr)].map(el => [el, arr.filter(v => v === el).length]));
console.log(Math.max(...map.values()));