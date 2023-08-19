const [h, s] = `${require('fs').readFileSync(0)}`
  .trim()
  .split(/\s+/)
  .slice(1)
  .reduce(([x, y], v) => [+v % 2 === 0 ? x + 1 : x, +v % 2 !== 0 ? y + 1 : y], [0, 0]);
console.log(h > s ? 'Happy' : 'Sad');