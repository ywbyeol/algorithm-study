const [n, ...i] = `${require('fs').readFileSync(0)}`.split('\n');
let cy = 100;
let sd = 100;
for (let j = 0; j < +n; j++) {
  const [a, b] = i[j].split(' ');
  +a > +b ? (sd -= +a) : +a < +b ? (cy -= +b) : 0;
}
console.log(`${cy}\n${sd}`);