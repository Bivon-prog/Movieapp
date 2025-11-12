# 📤 How to Upload to GitHub

## Step 1: Create a new repository on GitHub

1. Go to [github.com](https://github.com)
2. Click the "+" icon in the top right
3. Select "New repository"
4. Name it: `streambox` (or any name you prefer)
5. **DO NOT** initialize with README, .gitignore, or license (we already have these)
6. Click "Create repository"

## Step 2: Push your code

After creating the repository, run these commands:

```bash
# Add your GitHub repository as remote
git remote add origin https://github.com/YOUR_USERNAME/streambox.git

# Push your code
git branch -M main
git push -u origin main
```

Replace `YOUR_USERNAME` with your actual GitHub username.

## Alternative: Using SSH

If you have SSH keys set up:

```bash
git remote add origin git@github.com:YOUR_USERNAME/streambox.git
git branch -M main
git push -u origin main
```

## ✅ What's Already Done

- ✅ Git repository initialized
- ✅ All files committed
- ✅ .gitignore created (excludes .env, target/, etc.)
- ✅ README.md with full documentation
- ✅ LICENSE file (MIT)
- ✅ .env.example for others to use

## 🔒 Security Note

Your `.env` file with actual API keys is **NOT** included in git (it's in .gitignore).
This keeps your TMDb API keys private and secure.

## 📝 After Uploading

1. Update the README.md with your actual GitHub username
2. Add a screenshot of your app
3. Update the contact information
4. Consider adding GitHub Actions for CI/CD

## 🎉 That's it!

Your StreamBox project is ready to be shared with the world!
