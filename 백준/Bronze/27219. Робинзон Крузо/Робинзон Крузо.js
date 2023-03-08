process.stdin.on('data', i => {
  const num = Number(i.toString());
  console.log(`${'V'.repeat(Math.floor(num / 5))}${'I'.repeat(num % 5)}`);
});