const [, I, ...h] = `${require('fs').readFileSync(0)}`.trim().split(/\s+/);
console.log(h.sort()[+I - 1]);