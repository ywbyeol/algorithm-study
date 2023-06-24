const [B, ...A] = `${require('fs').readFileSync(0)}`.trim().split(/\s+/).reverse();
console.log(A.filter(v => v !== B && v.slice(0, B.length) === B).length);