const w = `${require('fs').readFileSync(0)}`[0];
console.log(w === 'S' ? 'HIGHSCHOOL' : w === 'C' ? 'MASTER' : +w === 2 ? '0611' : 'CONTEST');