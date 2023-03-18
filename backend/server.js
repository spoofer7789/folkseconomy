const express = require('express');
const bodyParser = require('body-parser');
const app = express();
const MongoClient = require('mongodb').MongoClient;
const uri = "mongodb://localhost:27017"; // Replace this with your MongoDB connection string if different

app.use(bodyParser.json());

let db;
let usersCollection;

MongoClient.connect(uri, { useUnifiedTopology: true }, (err, client) => {
  if (err) {
    console.error('Error connecting to MongoDB:', err);
    return;
  }
  db = client.db('myDatabase');
  usersCollection = db.collection('users');
  console.log('Connected to MongoDB');
});

app.post('/api', (req, res) => {
  const { username, publicKey } = req.body;

  if (!username || !publicKey) {
    res.status(400).json({ error: 'Username and public key are required' });
    return;
  }

  usersCollection.insertOne({ username, publicKey }, (err, result) => {
    if (err) {
      res.status(500).json({ error: 'Error saving user data' });
      return;
    }
    res.status(201).json({ message: 'User data saved successfully' });
  });
});

app.listen(3000, () => {
  console.log('Server listening on port 3000');
});
