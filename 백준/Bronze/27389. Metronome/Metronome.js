process.stdin.on('data', i =>
  console.log((Number(i.toString()) / 4).toFixed(2)),
);