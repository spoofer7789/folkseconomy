
 async function main() {
  // Create an IPFS instance
  const ipfs = await IPFS.create()
  // Create an OrbitDB instance
  const orbitdb = await OrbitDB.createInstance(ipfs)
  // Create a key-value store for usernames and public keys
  const keystore = await orbitdb.kvstore('keystore')
  // Create a feed for each user
  const userFeeds = {}
  // Add a new user to the keystore
  async function addUser(username, publicKey) {
    // Check if the username already exists
    const existingUser = await keystore.get(username)
    if (existingUser) {
      throw new Error(`Username '${username}' already exists`)
    }
    // Add the user to the keystore
    await keystore.put(username, publicKey)
    // Create a new feed for the user
    const userFeed = await orbitdb.feed(`${username}-feed`, {
      accessController: {
        write: [orbitdb.identity.id],
      },
    })
    // Add the user's feed to the userFeeds object
    userFeeds[username] = userFeed
    console.log(`Added user '${username}' with public key '${publicKey}'`)
  }

  // Add an entry to a user's feed
  async function addEntry(username, entry) {
    // Get the user's public key from the keystore
    const publicKey = await keystore.get(username)
    if (!publicKey) {
      throw new Error(`User '${username}' not found`)
    }

    // Check that the entry object contains a 'message' property
    if (!entry.message) {
      throw new Error(`Entry object for user '${username}' must contain a 'message' property`)
    }

    // Add the entry to the user's feed
    await userFeeds[username].add({
      message: entry.message,
      publicKey,
    })

    console.log(`Added entry for user '${username}' with message '${entry.message}'`)
  }

  // Example usage
  try {
    // Add a new user
    const username = 'Alice';
    const publicKey = 'abc123';
    await addUser(username, publicKey);
  
    // Add an entry to the user's feed
    const entry = { message: 'Hello world' };
    await addEntry(username, entry);
  
    // Attempt to add an entry to a non-existent user's feed
    const nonExistentUsername = 'Bob';
    const nonExistentEntry = { message: 'This should fail' }; 
    try {
      await addEntry(nonExistentUsername, nonExistentEntry);
    } catch (e) {
      console.error(`Error adding entry to non-existent user: ${e.message}`);
    }
  } catch (e) {
    console.error(e);
  }
}