process.stdin.on('data', i => {
  const num = Number(i.toString().trim());
  console.log('V'.repeat(Math.floor(num / 5)) + 'I'.repeat(num % 5));
});