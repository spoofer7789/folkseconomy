const express = require('express');
const bodyParser = require('body-parser');
const app = express();
const MongoClient = require('mongodb').MongoClient;
const uri = "mongodb://localhost:27017"; // Replace this with your MongoDB connection string if different
const orbitdbFunctions = require('database/orbitdatabase');
app.use(bodyParser.json());
// Add a new route for adding a user to OrbitDB
app.post('/orbitdb/addUser', async (req, res) => {
  const { username, publicKey } = req.body;

  if (!username || !publicKey) {
    res.status(400).json({ error: 'Username and public key are required' });
    return;
  }

  try {
    await orbitdbFunctions.addUser(username, publicKey);
    res.status(201).json({ message: 'User data saved successfully in OrbitDB' });
  } catch (err) {
    res.status(500).json({ error: `Error saving user data in OrbitDB: ${err.message}` });
  }
});

let db;
let usersCollection;

const { addUser, addEntry } = require('database/orbitdatabase');

(async () => {
  try {
    await main();
  } catch (e) {
    console.error(e);
  }
})();

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