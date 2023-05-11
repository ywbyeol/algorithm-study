const i = `${require('fs').readFileSync(0)}`.trim();
console.log(`${(i.match(/JOI/g) || []).length}\n${(i.match(/(?=(IOI))/g) || []).length}`);