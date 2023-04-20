const i = `${require('fs').readFileSync(0)}`.trim();
console.log(i.slice(0, -1) + i.slice(1));