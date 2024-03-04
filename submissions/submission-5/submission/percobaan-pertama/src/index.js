// 1234567

/*
    1234567
 */

const express = require('express')

const app = express()

app.get('/', (req, res) => {
    res.send('<h1>1234567</h1>')
})

app.listen(5000, () => {
    console.log('port 5000')
})