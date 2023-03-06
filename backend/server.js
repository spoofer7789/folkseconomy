const express = require('express');
const app = express();

app.get('/api', (req, res) => {
  res.status(200).send({
    username: '',
    password: ''
  })
  res.json({ message: 'Hello from the server!' });
});

app.listen(3000, () => {
  console.log('Server listening on port 3000');
});