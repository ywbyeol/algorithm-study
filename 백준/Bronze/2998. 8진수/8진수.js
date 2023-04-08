const obj = {'000': 0, '001': 1, '010': 2, '011': 3, 100: 4, 101: 5, 110: 6, 111: 7,};
const i = `${require('fs').readFileSync('/dev/stdin')}`.trim();
console.log((i.length % 3 === 0 ? i : '0'.repeat(3 - (i.length % 3)) + i).match(/[0-9]{3}/g).map(x => obj[x]).join(''));