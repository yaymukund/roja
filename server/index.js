const express = require('express');
const serveStatic = require('serve-static-throttle');
const morgan = require('morgan');

let app = express();
let speed = parseInt(process.argv[2], 10);

console.log(`Throttling to ${speed} kilobytes p s`);
app.use(morgan('tiny', { immediate: true }));
app.use(serveStatic('music', { throttle: 1024 * speed }))
app.listen(3000)
