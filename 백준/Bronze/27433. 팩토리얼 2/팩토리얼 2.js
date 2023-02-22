process.stdin.on('data', i => {
  const factorial = n => {
    if (n <= 1) return 1;
    let result = BigInt(n);
    for (let j = n - 1; j > 1; j--) result *= BigInt(j);
    return result;
  };
  console.log(String(factorial(i)));
});