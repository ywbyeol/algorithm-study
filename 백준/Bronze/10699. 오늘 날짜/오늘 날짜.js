const date = new Date();
const GMT_9 = date.getTime() + date.getTimezoneOffset() * 60000 + 32400000;
const today = new Date(GMT_9);
console.log(today.toISOString().split('T')[0]);