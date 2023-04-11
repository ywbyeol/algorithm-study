const i = `${require('fs').readFileSync(0)}`.trim().split(/\s+/g).slice(1);
console.log([...i].filter(x => +x !== i.indexOf(x) + 1).length);