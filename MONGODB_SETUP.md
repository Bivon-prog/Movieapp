# 🗄️ MongoDB Setup Guide

## Option 1: Install MongoDB Locally (Recommended for Development)

### Windows

1. **Download MongoDB Community Server**
   - Go to https://www.mongodb.com/try/download/community
   - Download the Windows installer
   - Run the installer and follow the setup wizard

2. **Start MongoDB**
   ```powershell
   # MongoDB should start automatically as a service
   # To check if it's running:
   net start MongoDB
   ```

3. **Verify Installation**
   ```powershell
   mongosh
   # You should see the MongoDB shell
   ```

### macOS

```bash
# Using Homebrew
brew tap mongodb/brew
brew install mongodb-community
brew services start mongodb-community
```

### Linux (Ubuntu/Debian)

```bash
# Import MongoDB public key
wget -qO - https://www.mongodb.org/static/pgp/server-6.0.asc | sudo apt-key add -

# Add MongoDB repository
echo "deb [ arch=amd64,arm64 ] https://repo.mongodb.org/apt/ubuntu focal/mongodb-org/6.0 multiverse" | sudo tee /etc/apt/sources.list.d/mongodb-org-6.0.list

# Install MongoDB
sudo apt-get update
sudo apt-get install -y mongodb-org

# Start MongoDB
sudo systemctl start mongod
sudo systemctl enable mongod
```

## Option 2: Use MongoDB Atlas (Cloud - Free Tier Available)

1. **Create Account**
   - Go to https://www.mongodb.com/cloud/atlas
   - Sign up for free

2. **Create Cluster**
   - Click "Build a Database"
   - Choose "FREE" tier (M0)
   - Select a cloud provider and region
   - Click "Create Cluster"

3. **Setup Database Access**
   - Go to "Database Access"
   - Click "Add New Database User"
   - Create username and password
   - Give "Read and write to any database" permission

4. **Setup Network Access**
   - Go to "Network Access"
   - Click "Add IP Address"
   - Click "Allow Access from Anywhere" (for development)
   - Or add your specific IP address

5. **Get Connection String**
   - Go to "Database" → "Connect"
   - Choose "Connect your application"
   - Copy the connection string
   - Replace `<password>` with your database user password

6. **Update .env file**
   ```env
   MONGODB_URI=mongodb+srv://username:password@cluster0.xxxxx.mongodb.net/?retryWrites=true&w=majority
   DATABASE_NAME=streambox
   ```

## Option 3: Use Docker

```bash
# Run MongoDB in Docker
docker run -d -p 27017:27017 --name mongodb mongo:latest

# Stop MongoDB
docker stop mongodb

# Start MongoDB
docker start mongodb
```

## Verify Connection

After setting up MongoDB, restart your StreamBox server:

```bash
cargo run
```

You should see:
```
🔌 Connecting to MongoDB at mongodb://localhost:27017
✅ MongoDB connected successfully!
```

## Test Authentication

1. Open http://localhost:8080
2. Click "Register"
3. Create an account
4. Try logging in

## Troubleshooting

### Connection Failed

**Error**: `MongoDB connection failed`

**Solutions**:
- Make sure MongoDB is running: `net start MongoDB` (Windows) or `brew services start mongodb-community` (macOS)
- Check if port 27017 is available
- Verify MONGODB_URI in .env file
- For Atlas: Check network access settings and connection string

### Authentication Error

**Error**: `Authentication failed`

**Solutions**:
- For Atlas: Verify username and password in connection string
- Check Database Access permissions
- Ensure IP address is whitelisted

## Default Configuration

The app will work without MongoDB, but authentication features will be disabled.

To enable authentication:
1. Install MongoDB (any option above)
2. Update .env with correct MONGODB_URI
3. Restart the server

## Database Structure

The app creates these collections automatically:
- `users` - User accounts with hashed passwords
- Indexes are created automatically for email (unique)

## Security Notes

- Passwords are hashed with bcrypt
- JWT tokens expire after 7 days
- Change JWT_SECRET in .env for production
- Never commit .env file to git (it's in .gitignore)
