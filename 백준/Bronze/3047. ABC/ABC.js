const [A, B, C, O] = `${require('fs').readFileSync(0)}`.split(/\W+/g);
const ABC = [A, B, C].sort((a, b) => a - b);
console.log(O.split('').map(x => ABC['ABC'.indexOf(x)]).join(' '));